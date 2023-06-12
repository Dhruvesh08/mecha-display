use std::io::{Error, Write, Read};

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
        file.write_all(adjusted_brightness.to_string().as_bytes())?;
        Ok(())
    }

    pub fn get_brightness(&self) -> Result<u8, Error> {
        let mut file = std::fs::File::open(&self.path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let brightness: u8 = buffer.trim().parse().unwrap();
        let adjusted_brightness = brightness * 100 / 255; // Convert 0-255 range to 0-100
        Ok(adjusted_brightness)
    }
}


