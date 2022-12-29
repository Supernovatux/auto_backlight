use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    ///Verbosity flag
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    ///Maximum and minimum change to brightness;
    #[clap(short, long, value_parser, default_value_t = 10)]
    pub limit: u8,
    ///Offset to limit.
    /// If offset is 5 and limit is 10 then brightness will change between -15 to 5;
    #[clap(long,short,allow_hyphen_values = true ,value_parser, default_value_t = -7)]
    pub offset: i16,
    ///Interval in which brightness values are refreshed in MillSecs
    #[clap(short, long, value_parser, default_value_t = 500)]
    pub refresh: u64,
}
impl Cli {
    ///Returns [log::Level] from verbosity flag passed via cli
    ///
    /// Defaults to [log::Level::Info]
    pub fn get_verbosity(&self) -> log::Level {
        self.verbose.log_level().unwrap()
    }
    ///Returns a i16. Which is used to determine the max change in brightness
    ///
    ///If the current brightness is 50%. And limit is 10. The brightness will vary between
    ///40%-60%
    pub fn get_limit(&self) -> u8 {
        self.limit
    }
    pub fn get_refresh(&self) -> u64 {
        self.refresh
    }
    pub fn get_offset(&self) -> i16 {
        self.offset
    }
}
