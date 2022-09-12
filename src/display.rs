use embedded_graphics_core::{
    geometry::Size,
    pixelcolor::{Bgr565, IntoStorage},
    prelude::*,
};
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, UsbContext};
use std::convert::TryInto;
use thiserror::Error;

pub struct NiDisplay {
    handle: DeviceHandle<Context>,
    frame_buffer: Box<[u16]>,
}

#[derive(Error, Debug)]
pub enum NiDisplayError {
    #[error("Device Not found")]
    DeviceNotFound,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    USBError(#[from] rusb::Error),
}

/* 3rd byte == screen idx, 0 or 1 */
const NI_HEADER: [u8; 16] = [
    0x84, 0x0, 0x01, 0x60, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0xe0, 0x1, 0x10,
];

/* num_px/2: 0xff00 is the (total_px/2) */
const NI_COMMAND: [u8; 4] = [0x00, 0x0, 0xff, 0x00];

const NI_FOOTER: [u8; 8] = [0x03, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00];

pub const DISPLAY_WIDTH: usize = 480;
pub const DISPLAY_HEIGHT: usize = 272;

const BYTES_PER_LINE: usize = 480 * 2;

const NI_VENDOR_ID: u16 = 0x17CC;
const MASCHINE_MK3_PRODUCT_ID: u16 = 0x1600;
const NI_MASCHINE_MK3_USBHID_INTERFACE: u8 = 5;
const NI_MASCHINE_BULK_EP_OUT: u8 = 0x04;

// TODO hvh: fixup how this works?
const MASK: [u8; 4] = [0xe7, 0xf3, 0xe7, 0xff];

impl NiDisplay {
    pub fn new() -> Result<NiDisplay, NiDisplayError> {
        let mut context = Context::new()?;
        let (_, _, mut handle) = open_device(&mut context, NI_VENDOR_ID, MASCHINE_MK3_PRODUCT_ID)
            .ok_or(NiDisplayError::DeviceNotFound)?;
        handle.claim_interface(NI_MASCHINE_MK3_USBHID_INTERFACE)?;

        let buffer: Box<[u16]> = vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT].into_boxed_slice();

        Ok(NiDisplay {
            handle,
            frame_buffer: buffer,
        })
    }

    /// Writes the frame buffer to the display. If no frame arrives in 2 seconds, the display is turned black
    pub fn flush(&self) -> Result<(), NiDisplayError> {
        use std::time::Duration;
        let timeout = Duration::from_secs(1);

        let tranfer_buffer = self.masked_frame_buffer();
        self.handle
            .write_bulk(NI_MASCHINE_BULK_EP_OUT, &NI_HEADER, timeout)?;
        self.handle
            .write_bulk(NI_MASCHINE_BULK_EP_OUT, &NI_COMMAND, timeout)?;
        self.handle
            .write_bulk(NI_MASCHINE_BULK_EP_OUT, &tranfer_buffer, timeout)?;
        self.handle
            .write_bulk(NI_MASCHINE_BULK_EP_OUT, &NI_FOOTER, timeout)?;

        Ok(())
    }

    fn masked_frame_buffer(&self) -> [u8; BYTES_PER_LINE * DISPLAY_HEIGHT] {
        let mut masked_buffer: [u8; BYTES_PER_LINE * DISPLAY_HEIGHT] =
            [0; BYTES_PER_LINE * DISPLAY_HEIGHT];
        for r in 0..DISPLAY_HEIGHT {
            for c in 0..DISPLAY_WIDTH {
                let i = r * DISPLAY_WIDTH + c;
                let b: [u8; 2] = u16::to_le_bytes(self.frame_buffer[i]);
                let di = r * BYTES_PER_LINE + c * 2;
                masked_buffer[di] = b[0] ^ MASK[di % 4];
                masked_buffer[di + 1] = b[1] ^ MASK[(di + 1) % 4];
            }
        }

        masked_buffer
    }
}

impl DrawTarget for NiDisplay {
    type Color = Bgr565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Ok((x @ 0..=479, y @ 0..=271)) = point.try_into() {
                let index: u32 = x + y * 480;
                self.frame_buffer[index as usize] = color.into_storage();
            }
        }

        Ok(())
    }
}

impl OriginDimensions for NiDisplay {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
    }
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}
