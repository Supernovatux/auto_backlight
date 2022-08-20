//use std::{env, fs};

use auto_backlight::{
    brightness::BrightnessDevices, cli_parser::{get_limit, get_refresh}, init, screens::change_calc,
};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    init();
    let mut brightness = 0;
    let mut change = 0;
    let brightness_dev = BrightnessDevices::new();
    loop {
        let change_new = change_calc(get_limit() as u8);
        if change != change_new {
            //User has changed brightness
            change = change_new;
            if brightness == brightness_dev.get_brightness() {
                brightness = brightness_dev.get_brightness();
            } else {
                brightness_dev.set_brightness(change);
            }
        }
        sleep(Duration::from_secs(get_refresh()));
    }
}
