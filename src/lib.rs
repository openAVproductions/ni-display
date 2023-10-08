//! This Rust library implements the Embedded-graphics-core `DrawTarget` trait,
//! making it easy to draw 2D graphics primitives on the Native Instruments display.
//!
//! Native Instruments Maschine Mk3 is a USB-HID controller, with a USB-Bulk endpoint driving 2x 480x272 RGB LCD displays.
//!
//! # Examples
//!
//! Checkout the repo examples/demo.rs for a short easy demo.

pub mod display;
pub use display::*;
