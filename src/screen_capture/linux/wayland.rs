use dbus::{self, blocking::Connection};

use std::{
    env::temp_dir,
    fs,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
fn screenshot() -> Result<String, dbus::Error> {
    let conn = Connection::new_session()?;

    let proxy = conn.with_proxy(
        "org.gnome.Shell.Screenshot",
        "/org/gnome/Shell/Screenshot",
        Duration::from_secs(10),
    );

    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_micros().to_string(),
        Err(_) => return Err(dbus::Error::new_failed("Get system timestamp failed")),
    };

    let dirname = temp_dir().join("screenshot");

    fs::create_dir_all(&dirname).map_err(|_| {
        dbus::Error::new_failed(format!("Create dir {:?} failed", dirname).as_str())
    })?;

    let mut path = dirname.join(timestamp);
    path.set_extension("png");

    let filename = path.to_string_lossy().to_string();

    proxy.method_call(
        "org.gnome.Shell.Screenshot",
        "Screenshot",
        (false, false, &filename),
    )?;

    Ok(filename)
}
pub fn wayland_capture_screen_raw() -> Option<Vec<u8>> {
    let filename = screenshot().ok()?;
    std::fs::read(filename).ok()
}
