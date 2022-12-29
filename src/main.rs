use auto_backlight::cli_parser;
use clap::Parser;
use file_lock::{FileLock, FileOptions};

fn main() {
    let args = cli_parser::Cli::parse();
    let options = FileOptions::new().write(true).read(true).create(true);
    let filelock = match FileLock::lock("/tmp/auto-backlight.lock", false, options) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {err}"),
    };
    auto_backlight::init(args);
    filelock.unlock().unwrap()
}
