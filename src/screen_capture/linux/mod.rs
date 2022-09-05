use std::env::var_os;

use display_info::DisplayInfo;

use self::{wayland::wayland_capture_screen_raw, xorg::xorg_capture_screen_raw};

mod wayland;
mod xorg;
fn wayland_detect() -> bool {
    let xdg_session_type = var_os("XDG_SESSION_TYPE")
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let wayland_display = var_os("WAYLAND_DISPLAY")
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    xdg_session_type.eq("wayland") || wayland_display.to_lowercase().contains("wayland")
}
pub fn capture_screen_raw(display_info: &DisplayInfo) -> Option<Vec<u8>> {
    if wayland_detect() {
        wayland_capture_screen_raw()
    } else {
        xorg_capture_screen_raw(display_info)
    }
}
