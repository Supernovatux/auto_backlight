use glob::glob;
use log::{debug, error, info};

use std::{
    fs::{read_to_string, write},
    io::Error,
};

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
                    let max_brightness = match read_to_string(format!(
                        "{}/max_brightness",
                        path.to_str().unwrap()
                    )) {
                        Ok(sm) => sm.trim().parse().unwrap(),
                        Err(_) => continue,
                    };
                    let new_dev = BrightnessDevice {
                        max_brightness,
                        brightness: format!("{}/brightness", path.to_str().unwrap()),
                    };
                    devices.push(new_dev);
                }
                Err(e) => error!("Glob error {:?}", e),
            }
        }
        BrightnessDevices { devices }
    }
    pub fn get_brightness(&mut self) -> i16 {
        //Multi-monitor support is to be added
        match self.devices[0].get_current_brightness_percent() {
            Ok(num) => num,
            Err(err) => {
                error!("{}", err);
                self.devices.remove(0);
                0
            }
        }
    }
    pub fn change_brightness(&mut self, change: i16) {
        match self.devices[0].increase_brightness(change) {
            Ok(_) => (),
            Err(err) => {
                error!("{:?}", err);
                self.devices.remove(0);
            }
        }
    }
}

#[derive(Debug)]
pub struct BrightnessDevice {
    max_brightness: i16,
    brightness: String,
}

impl BrightnessDevice {
    fn get_max_brightness(&self) -> i16 {
        self.max_brightness
    }
    fn get_current_brightness(&self) -> Result<i16, Error> {
        match read_to_string(&self.brightness) {
            Ok(num) => Ok(num.trim().parse().unwrap()),
            Err(err) => Err(err),
        }
    }
    pub fn get_current_brightness_percent(&self) -> Result<i16, Error> {
        let ret = (self.get_current_brightness()? as f64 * 100.0 / self.get_max_brightness() as f64)
            as i16;
        debug!("Current brightness is {}", ret);
        Ok(ret)
    }
    pub fn increase_brightness(&self, change: i16) -> Result<(), Error> {
        let change = (change as f64 * self.get_max_brightness() as f64 / 100.0) as i16;
        let value = self.get_current_brightness()?;
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
        write(&self.brightness, format!("{value_new}" )).expect("permission denied");
        Ok(())
    }
}

impl Default for BrightnessDevices {
    fn default() -> Self {
        Self::new()
    }
}
