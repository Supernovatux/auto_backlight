use crossbeam_channel::Sender;
use ksni;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct SysTray {
    running: Arc<AtomicBool>,
    tx: crossbeam_channel::Sender<Option<i32>>,
}

impl ksni::Tray for SysTray {
    fn icon_name(&self) -> String {
        if self.running.load(Ordering::Relaxed) {
            "display-brightness-high-symbolic".into()
        } else {
            "display-brightness-off-symbolic".into()
        }
    }
    fn activate(&mut self, _x: i32, _y: i32) {
        if self.running.load(Ordering::Relaxed) {
            self.running.store(false, Ordering::Relaxed)
        } else {
            self.running.store(true, Ordering::Relaxed)
        }
    }
    fn title(&self) -> String {
        "auto_backlight".into()
    }
    fn tool_tip(&self) -> ksni::ToolTip {
        let description = if self.running.load(Ordering::Relaxed) {
            "Auto backlight is running".into()
        } else {
            "Auto backlight is not running".into()
        };
        ksni::ToolTip {
            title: "auto_backlight".into(),
            description,
            ..Default::default()
        }
    }
    fn category(&self) -> ksni::Category {
        ksni::Category::Hardware
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            CheckmarkItem {
                label: "Enabled".into(),
                checked: self.running.load(Ordering::Relaxed),
                activate: Box::new(|this: &mut Self| {
                    if this.running.load(Ordering::Relaxed) {
                        this.running.store(false, Ordering::Relaxed)
                    } else {
                        this.running.store(true, Ordering::Relaxed)
                    }
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|this: &mut Self| {
                    this.running.store(false, Ordering::Relaxed);
                    this.tx.send(None).unwrap();
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub fn start_knsi(status: Arc<AtomicBool>, tx: Sender<Option<i32>>) -> ksni::Handle<SysTray> {
    let service = ksni::TrayService::new(SysTray {
        running: status,
        tx,
    });
    let ret = service.handle();
    service.spawn();
    ret
}
