use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use futures::channel::oneshot;
use ksni;

pub struct SysTray {
    running: Arc<AtomicBool>,
    tx: oneshot::Sender<()>,
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
                    let (mut tx, _rx) = oneshot::channel();
                    mem::swap(&mut this.tx, &mut tx);
                    tx.send(()).unwrap();
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub fn start_knsi(status: Arc<AtomicBool>, tx: oneshot::Sender<()>) -> ksni::Handle<SysTray> {
    let service = ksni::TrayService::new(SysTray {
        running: status,
        tx,
    });
    let ret = service.handle();
    service.spawn();
    ret
}
#[cfg(test)]
mod test {
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread,
        time::Duration,
    };

    use futures::channel::oneshot;

    use super::start_knsi;

    #[test]
    fn does_it_work() {
        let (tx, _) = oneshot::channel();
        let new = Arc::new(AtomicBool::new(true));
        let new2 = new.clone();
        let _handle1 = thread::spawn(move || start_knsi(new, tx));
        loop {
            println!("{}", new2.load(Ordering::Relaxed));
            thread::sleep(Duration::from_secs(2));
        }
    }
}
