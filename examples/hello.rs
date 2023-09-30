//! # Example: Hello
//!
//! A simple example displaying some graphics and text on the display.

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Bgr565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use ni_display::NiDisplay;
use std::{error, thread, time};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut display = NiDisplay::new().unwrap();
    let text_style = MonoTextStyle::new(&FONT_10X20, Bgr565::WHITE);

    display.select_display(0).unwrap();

    let mut position = Point::new(0, 120);
    let mut step = 4;
    loop {
        display.clear(Bgr565::BLACK)?;

        Rectangle::new(Point::zero(), display.size() / 4)
            .into_styled(PrimitiveStyle::with_fill(Bgr565::RED))
            .draw(&mut display)?;

        Rectangle::new(Point::new(150, 0), display.size() / 4)
            .into_styled(PrimitiveStyle::with_fill(Bgr565::GREEN))
            .draw(&mut display)?;

        Rectangle::new(Point::new(300, 0), display.size() / 4)
            .into_styled(PrimitiveStyle::with_fill(Bgr565::BLUE))
            .draw(&mut display)?;

        for (x_off, col) in [
            Bgr565::BLUE,
            Bgr565::MAGENTA,
            Bgr565::RED,
            Bgr565::CYAN,
            Bgr565::YELLOW,
            Bgr565::WHITE,
        ]
        .iter()
        .enumerate()
        {
            Rectangle::new(
                Point::new(0, 100 + 30 * x_off as i32),
                Size::new(display.size().width, 16),
            )
            .into_styled(PrimitiveStyle::with_stroke(col.clone(), 2))
            .draw(&mut display)?;
        }

        position.x += step;
        if position.x >= display.size().width as i32 || position.x <= 0 {
            step *= -1;
        }

        Text::new("Hello!", position, text_style).draw(&mut display)?;

        display.flush()?; // if no frame arrives in 2 seconds, the display is turned black
        thread::sleep(time::Duration::from_millis(1000 / 60));
    }
}
