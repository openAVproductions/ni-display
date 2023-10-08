# ni_display

[![Crate](https://img.shields.io/crates/v/ni_display.svg)](https://crates.io/crates/ni_display)
[![API](https://docs.rs/ni_display/badge.svg)](https://docs.rs/ni_display)

This Rust library implements the Embedded-graphics-core `DrawTarget` trait,
making it easy to draw 2D graphics primitives on the Maschine Mk3 display.

## Examples

Clone this repo, see examples/demo.rs, and run
```bash
cargo run --example demo
```

## References
This repo is inspired from, and based on code of the "push2_display" crate
by Marc Bracher. Thank you Marc for sharing your code and implementation, converting
the Push2 code to NativeInstruments was fun - and I would not have built the whole
rust based embedded-graphics infrastructure myself. Checkout his code here:
[Ableton Push2 embedded graphics repo](https://github.com/mbracher/push2_display)

Also, the embedded graphics project is the ecosystem that enabled this crate,
checkout the great work there, and use the UI/pixel manipulating code from here:
[Embedded graphics](https://github.com/embedded-graphics/embedded-graphics)

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
