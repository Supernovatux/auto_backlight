use auto_backlight::{
    brightness::BrightnessDevices,
    cli_parser::{get_limit, get_refresh},
    init,
    screens::change_calc,
    sys_tray,
};
use log::info;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::oneshot;
#[tokio::main]
async fn main() {
    init();
    let (tx, mut rx) = oneshot::channel();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(get_refresh()));
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
        tokio::select! {
            _ = interval.tick() => info!("Current brightness {}",brightness),
            _ = &mut rx => {
                brightness_dev.change_brightness(-change);
                break;},
        }
    }
    handle.shutdown();
}
