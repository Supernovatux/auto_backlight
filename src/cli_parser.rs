use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Cli {
    ///Verbosity flag
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    ///Maximum and minimum change to brightness;
    #[clap(short, long, value_parser, default_value_t = 10)]
    pub limit: i16,
    ///Offset to limit. Use as --offset=<Value>
    /// if limit=10
    /// offset = -5
    /// then brightness will vary between -15 to 5;
    #[clap(long, value_parser, default_value_t = -7)]
    pub offset: i16,
    ///Interval in which brightness values are refreshed
    #[clap(short, long, value_parser, default_value_t = 5)]
    pub refresh: u64,
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
fn init_cli() -> Cli {
    Cli::parse()
}
