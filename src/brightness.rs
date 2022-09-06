use glob::glob;
use log::{debug, error, info};

use std::fs::{read_to_string, write};

#[derive(Debug)]
pub struct BrightnessDevices {
    devices: Vec<BrightnessDevice>,
}

impl BrightnessDevices {
    pub fn new() -> Self {
        let mut devices = Vec::new();
        for entry in glob("/sys/class/backlight/*").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let new_dev = BrightnessDevice {
                        max_brightness: format!("{}/max_brightness", path.to_str().unwrap()),
                        brightness: format!("{}/brightness", path.to_str().unwrap()),
                    };
                    devices.push(new_dev);
                }
                Err(e) => error!("Glob error {:?}", e),
            }
        }
        BrightnessDevices { devices }
    }
    pub fn get_brightness(&self) -> i16 {
        //As of now it averages out.
        //Multi-monitor support is to be addeed
        let sum: i16 = self
            .devices
            .iter()
            .map(|dev| dev.get_current_brightness_percent())
            .sum();
        sum / self.devices.len() as i16
    }
    pub fn change_brightness(&self, change: i16) {
        self.devices[0].increase_brightness(change)
    }
}

#[derive(Debug)]
pub struct BrightnessDevice {
    pub max_brightness: String,
    pub brightness: String,
}

impl BrightnessDevice {
    fn get_max_brightness(&self) -> i16 {
        read_to_string(&self.max_brightness)
            .unwrap()
            .trim()
            .parse()
            .unwrap()
    }
    fn get_current_brightness(&self) -> i16 {
        read_to_string(&self.brightness)
            .unwrap()
            .trim()
            .parse()
            .unwrap()
    }
    pub fn get_current_brightness_percent(&self) -> i16 {
        let ret = (self.get_current_brightness() as f64 * 100.0 / self.get_max_brightness() as f64)
            as i16;
        debug!("Current brightness is {}", ret);
        ret
    }
    pub fn increase_brightness(&self, change: i16) {
        let change = (change as f64 * self.get_max_brightness() as f64 / 100.0) as i16;
        let value = self.get_current_brightness();
        let value_new = if change == 0 {
            value
        } else if value + change < 0 {
            0
        } else if value + change > self.get_max_brightness() {
            100
        } else {
            value + change
        };
        info!("Brightness changed from {} to {}", value, value_new);
        write(&self.brightness, format!("{}", value_new)).expect("permission denied");
    }
}

impl Default for BrightnessDevices {
    fn default() -> Self {
        Self::new()
    }
}
