use std::io::{Error};

use display::DisplayBrightness;
mod display;


fn main() -> Result<(), Error> {

    //get value form args
    let args: Vec<String> = std::env::args().collect();
    let brightness: u8 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid brightness value",
            ))
        }
    };
    let display_brightness = DisplayBrightness::new();
    display_brightness.set_brightness(brightness)?;

    println!("Brightness: {}", display_brightness.get_brightness()?);

    //sleep for 0.5 seconds`

    Ok(())
}