use embedded_graphics::{
    pixelcolor::Bgr565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use ni_display::NiDisplay;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut display = NiDisplay::new()?;


    for _ in 0..2 {
        for color in [Bgr565::RED, Bgr565::GREEN, Bgr565::BLUE] {
            for screen in 0..2 {
                display.select_display(screen)?;
                println!("screen {screen} is now {:?}", color);

                Rectangle::new(Point::new(480/4, 272/4), display.size()/2)
                    .into_styled(PrimitiveStyle::with_fill(color))
                    .draw(&mut display)?;
                display.flush()?;
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }

    // screens are cleared on Drop
    drop(display);

    println!("demo done!");
    Ok(())
}
