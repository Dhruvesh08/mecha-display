mod display;
use display::{Display, DisplayInterface};

fn main() {
    let mut display = Display {
        path: String::new(),
    };

    display.set_device("/sys/class/backlight/backlight/brightness");
    display.set_brightness(50).unwrap();
    display.info();
    let brightness = display.get_brightness().unwrap();
    println!("Brightness: {}", brightness);
    println!("Device: {}", display.get_device());
}
