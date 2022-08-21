use auto_backlight::{
    brightness::BrightnessDevices,
    cli_parser::{get_limit, get_refresh},
    init,
    screens::change_calc,
    sys_tray,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    init();
    let brightnessctl_status = Arc::new(AtomicBool::new(true));
    let status_to_send = brightnessctl_status.clone();
    let mut brightness = 0;
    let mut change = 0;
    let brightness_dev = BrightnessDevices::new();
    thread::spawn(move || sys_tray::start_knsi(status_to_send));
    loop {
        if brightnessctl_status.load(Ordering::Relaxed) {
            let change_new = change_calc(get_limit() as u8);
            if change != change_new {
                //User has changed brightness
                if brightness != brightness_dev.get_brightness() + change {
                    brightness = brightness_dev.get_brightness();
                    brightness_dev.set_brightness(-change + change_new);
                } else {
                    brightness_dev.set_brightness(-change + change_new);
                }
                change = change_new;
            }
        } else if change != 0 {
            brightness_dev.set_brightness(-change);
            change = 0;
        }
        sleep(Duration::from_secs(get_refresh()));
    }
}
