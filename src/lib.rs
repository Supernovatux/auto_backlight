use cli_parser::Cli;
use crossbeam_channel::{bounded, select, tick, Sender};
use log::{debug, info};
use signal_hook::{
    consts::TERM_SIGNALS,
    iterator::{exfiltrator::SignalOnly, SignalsInfo},
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::{brightness::BrightnessDevices, screens::change_calc};
pub mod brightness;
pub mod cli_parser;
pub mod screen_capture;
pub mod screens;
pub mod sys_tray;

pub fn init(args: Cli) {
    simple_logger::SimpleLogger::new()
        .with_level(args.get_verbosity().to_level_filter())
        .without_timestamps()
        .init()
        .unwrap();
    let (tx, rx) = bounded(16);
    let delay = tick(Duration::from_millis(args.get_refresh()));
    let brightnessctl_status = Arc::new(AtomicBool::new(true));
    let status_to_send = brightnessctl_status.clone();
    let mut brightness = 0;
    let mut change = 0;
    let mut brightness_dev = BrightnessDevices::new();
    let handle = sys_tray::start_knsi(status_to_send, tx.clone());
    thread::spawn(|| {
        sig_handle(tx);
    });
    loop {
        if brightnessctl_status.load(Ordering::Relaxed) {
            let change_new = change_calc(args.get_limit(), args.get_offset());
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
            recv(rx) -> msg => {
                brightness_dev.change_brightness(-change);
                if cfg!(debug_assertions) {
                    match msg {
                        Ok(msg) => {
                            match msg {
                                Some(msg) => info!("Received exit signal {}",msg),
                                None => info!("Received taskbar exit signal"),
                            }
                        },
                        Err(err) => log::error!("{err}"),
                    };
                }
                info!("Exiting");
                break;
            },
        }
    }
    handle.shutdown();
}

fn sig_handle(tx: Sender<Option<i32>>) {
    let sigs = Vec::from(TERM_SIGNALS);
    let mut signals = SignalsInfo::<SignalOnly>::new(sigs).unwrap();
    if let Some(info) = (&mut signals).into_iter().next() {
        tx.send(Some(info)).unwrap();
    }
}
