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
      
        let mut file = std::fs::File::create(&self.path)?;

        println!("Set Brightness: {}", brightness);
       
        file.write_all(brightness.to_string().as_bytes())?;
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
        println!("Get Brightness: {}", brightness);
        Ok(brightness)
    }
}


