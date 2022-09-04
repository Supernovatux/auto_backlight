use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use futures::{channel::oneshot, FutureExt};
use futures_timer::Delay;
use log::{info, debug};

use crate::{cli_parser::{get_refresh, get_limit}, brightness::BrightnessDevices, screens::change_calc};
pub mod brightness;
pub mod cli_parser;
pub mod screens;
pub mod sys_tray;
pub async fn init() {
    let log_lev = cli_parser::get_verbosity();
    simple_logger::init_with_level(log_lev).unwrap();
    let refresh = get_refresh();
    info!("Starting with log_lev:- {:?}", log_lev);
    let (tx, mut rx) = oneshot::channel();
    let brightnessctl_status = Arc::new(AtomicBool::new(true));
    let status_to_send = brightnessctl_status.clone();
    let mut brightness = 0;
    let mut change = 0;
    let brightness_dev = BrightnessDevices::new();
    let handle = sys_tray::start_knsi(status_to_send, tx);
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
        futures::select! {
            _ =  Delay::new(std::time::Duration::from_secs(refresh)).fuse() => debug!("Current brightness {}",brightness),
            _ = &mut rx => {
                brightness_dev.change_brightness(-change);
                info!("Got exit signal");
                break;},
        }
    }
    handle.shutdown();
}
