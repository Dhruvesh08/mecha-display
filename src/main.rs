use std::io::{Error};

use display::DisplayBrightness;
mod display;


fn main() -> Result<(), Error> {
    let display_brightness = DisplayBrightness::new();
    display_brightness.set_brightness(50)?;

    println!("Brightness: {}", display_brightness.get_brightness()?);

    display_brightness.set_brightness(100)?;
    println!("Brightness: {}", display_brightness.get_brightness()?);
    Ok(())
}