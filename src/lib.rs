use crossbeam_channel::{bounded, tick, select};
use log::{debug, info};
use std::{sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
}, time::Duration};

use crate::{
    brightness::BrightnessDevices,
    cli_parser::{get_limit, get_refresh},
    screens::change_calc,
};
pub mod brightness;
pub mod cli_parser;
pub mod screen_capture;
pub mod screens;
pub mod sys_tray;

pub fn init() {
    let log_lev = cli_parser::get_verbosity();
    simple_logger::SimpleLogger::new()
        .with_level(log_lev.to_level_filter())
        .without_timestamps()
        .init()
        .unwrap();
    info!("Starting with log_lev:- {:?}", log_lev);
    let (tx, rx) = bounded(100);
    let delay = tick(Duration::from_millis(get_refresh()));
    let brightnessctl_status = Arc::new(AtomicBool::new(true));
    let status_to_send = brightnessctl_status.clone();
    let mut brightness = 0;
    let mut change = 0;
    let brightness_dev = BrightnessDevices::new();
    let handle = sys_tray::start_knsi(status_to_send, tx.clone());
    loop {
        if brightnessctl_status.load(Ordering::Relaxed) {
            let change_new = change_calc(get_limit() as u8);
            if change != change_new {
                //User has changed brightness
                if brightness != brightness_dev.get_brightness() + change {
                    brightness = brightness_dev.get_brightness();
                    brightness_dev.change_brightness(-change + change_new);
                } else {
                    brightness_dev.change_brightness(-change + change_new);
                }
                change = change_new;
            }
        } else if change != 0 {
            brightness_dev.change_brightness(-change);
            change = 0;
        }
        select! {
            recv(delay) -> _ => debug!("Current brightness {}",brightness),
            recv(rx) -> _ => {
                brightness_dev.change_brightness(-change);
                info!("Got exit signal");
                break;},
        }
    }
    handle.shutdown();
}
