# ni_display

[![Crate](https://img.shields.io/crates/v/ni_display.svg)](https://crates.io/crates/ni_display)
[![API](https://docs.rs/ni_display/badge.svg)](https://docs.rs/ni_display)

This Rust library implements the Embedded-graphics-core `DrawTarget` trait,
making it easy to draw 2D graphics primitives on the Maschine Mk3 display.

## Examples

```rust
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::{PixelColor, Bgr565},
    prelude::*,
    text::Text,
};
use ni_display::NiDisplay;

let mut display = NiDisplay::new().unwrap();
let text_style = MonoTextStyle::new(&FONT_10X20, Bgr565::WHITE);

Text::new("Hello!", Point::new(400, 70), &text_style)
    .draw(&mut display)?;

display.flush()?;
```

```bash
git clone https://github.com/mbracher/push2_display
cd push2_display

cargo run --example hello
```

![Photo of hello example on Push2 device](https://raw.githubusercontent.com/mbracher/push2_display/master/doc/assets/push2hello.jpg)

## References
[Ableton Push Interface](https://github.com/Ableton/push-interface)

[Embedded graphics](https://github.com/embedded-graphics/embedded-graphics)

## Projects using push2_display
- [push2_pong](https://github.com/mbracher/push2_pong): two player ping pong game on the Ableton Push2 midi controller

- [push2_soundboard](https://github.com/Dragonseel/push2_soundboard): play sounds and loops via pressing buttons on the Ableton Push2 midi controller

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
