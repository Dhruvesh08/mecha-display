use std::io::{Error};

use display::DisplayBrightness;
mod display;

fn main() -> Result<(), Error> {
    let display_brightness = DisplayBrightness::new();
    display_brightness.set_brightness(50)?;

    print!("Current brightness: {}%", display_brightness.get_brightness()?);
    Ok(())
}