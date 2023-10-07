//! This Rust library implements the Embedded-graphics-core `DrawTarget` trait,
//! making it easy to draw 2D graphics primitives on the Native Instruments display.
//!
//! Native Instruments Maschine Mk3 is a USB-HID controller, with a USB-Bulk endpoint driving 2x 480x272 RGB LCD displays.
//!
//! # Examples
//!
//! ```rust
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_10X20, MonoTextStyle},
//!     pixelcolor::{PixelColor, Bgr565},
//!     prelude::*,
//!     text::Text,
//! };
//! use push2_display::Push2Display;
//!
//! let mut display = Push2Display::new()?;
//! let text_style = MonoTextStyle::new(&FONT_10X20, Bgr565::WHITE);
//!
//! Text::new("Hello!", Point::new(400, 70), &text_style)
//!     .draw(&mut display)?;
//!
//! display.flush()?;
//! ```
//!
//! ```bash
//! git clone https://github.com/mbracher/push2_display
//! cd push2_display
//!
//! cargo run --example hello
//! ```
//!
//! ![Photo of hello example on Push2 device](https://raw.githubusercontent.com/mbracher/push2_display/master/doc/assets/push2hello.jpg)
//!
//! # References
//! [Ableton Push Interface](https://github.com/Ableton/push-interface)
//!
//! [Embedded graphics](https://github.com/embedded-graphics/embedded-graphics)
//!
//! # Projects using push2_display
//! - [push2_pong](https://github.com/mbracher/push2_pong): two player ping pong game on the Ableton Push2 midi controller
//!
//! - [push2_soundboard](https://github.com/Dragonseel/push2_soundboard): play sounds and loops via pressing buttons on the Ableton Push2 midi controller

pub mod display;
pub use display::*;
