use std::fs;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::info;
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Cli {
    ///Verbosity flag
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    ///Path to store temp images
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = get_def_path(),
    )]
    pub path: String,
    ///Maximum and minimum change to brightness;
    #[clap(short, long, value_parser, default_value_t = 20)]
    pub limit: i16,
    ///Offset to limit
    /// if limit=10
    /// offset = 5
    /// then brightness will vary between -15 to 5;
    #[clap(short, long, value_parser, default_value_t = 16)]
    pub offset: i16,
    ///Interval in which brightness values are refreshed
    #[clap(short, long, value_parser, default_value_t = 5)]
    pub refresh: u64,
}
///Return path to password as [String]
///
/// If path was not provided by cli defaults to `~/.config/pass`
pub fn get_path() -> String {
    let arg = init_cli();
    arg.path
}
///Returns [log::Level] from verbosity flag passed via cli
///
/// Defaults to [log::Level::Info]
pub fn get_verbosity() -> log::Level {
    let arg = init_cli();
    arg.verbose.log_level().unwrap()
}
///Returns a i16. Which is used to determine the max change in brightness
///
///If the current brightness is 50%. And limit is 10. The brightness will vary between
///40%-60%
pub fn get_limit() -> i16 {
    let arg = init_cli();
    arg.limit
}
pub fn get_refresh() -> u64 {
    let arg = init_cli();
    arg.refresh
}
pub fn get_offset() -> i16 {
    let arg = init_cli();
    arg.offset
}
fn get_def_path() -> String {
    fs::create_dir_all("/tmp/auto_backlight/screenshots").unwrap();
    info!("Created /tmp/auto_backlight/screenshots");
    String::from("/tmp/auto_backlight/screenshots/")
}
fn init_cli() -> Cli {
    Cli::parse()
}
