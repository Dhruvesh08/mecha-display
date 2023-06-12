use std::io::{Error, ErrorKind, Write, Read};

pub struct DisplayBrightness {
    path: String,
}

impl DisplayBrightness {
    pub fn new() -> DisplayBrightness {
        DisplayBrightness {
            path: String::from("/sys/class/backlight/backlight/brightness"),
        }
    }

    pub fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        let adjusted_brightness = brightness * 255 / 100; // Convert 0-100 range to 0-255
        let mut file = std::fs::File::create(&self.path)?;
        println!("Setting brightness to: {} (input: {}) \n", adjusted_brightness, brightness);
        println!("Writing to file: {}\n", self.path);
        println!("Brightness: {}\n", adjusted_brightness);
        file.write_all(adjusted_brightness.to_string().as_bytes())?;
        Ok(())
    }
    

    pub fn get_brightness(&self) -> Result<u8, Error> {
        let mut file = std::fs::File::open(&self.path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let brightness: u8 = match buffer.trim().parse() {
            Ok(value) => value,
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Invalid brightness value")),
        };
        let adjusted_brightness = brightness * 100 / 255 + 1; // Convert 0-255 range to 1-100
        Ok(adjusted_brightness)
    }
}

