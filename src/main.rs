use std::io::{Error};

use display::DisplayBrightness;
mod display;


fn main() -> Result<(), Error> {
    let display_brightness = DisplayBrightness::new();
    display_brightness.set_brightness(50)?;

    println!("Brightness: {}", display_brightness.get_brightness()?);

    //sleep for 0.5 seconds`
    std::thread::sleep(std::time::Duration::from_millis(500));

    display_brightness.set_brightness(100)?;
    println!("Brightness: {}", display_brightness.get_brightness()?);
    Ok(())
}