pub use display_info::DisplayInfo;
mod linux;
use linux::*;

#[derive(Debug, Clone, Copy)]
pub struct Screen {
    pub display_info: DisplayInfo,
}

impl Screen {
    pub fn new(display_info: &DisplayInfo) -> Self {
        Screen {
            display_info: *display_info,
        }
    }

    pub fn all() -> Option<Vec<Screen>> {
        let screens = DisplayInfo::all()?.iter().map(Screen::new).collect();
        Some(screens)
    }

    pub fn capture_raw(&self) -> Option<Vec<u8>> {
        capture_screen_raw(&self.display_info)
    }
}
