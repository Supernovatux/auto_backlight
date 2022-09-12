use std::fs::{File, self};

fn main() {
    if File::open("/tmp/auto-backlight.lock").is_ok() {
        eprintln!("Is another instance of the process running?");
        eprintln!("If not then try deleting /tmp/auto-backlight.lock");
        return;
    }
    if File::create("/tmp/auto-backlight.lock").is_err() {
        eprintln!("Failed to create lock file.");
        eprintln!("Exiting");
        return;
    }
    auto_backlight::init();
    if fs::remove_file("/tmp/auto-backlight.lock").is_err() {
        eprintln!("Failed to remove the lockfile");
    }
}
