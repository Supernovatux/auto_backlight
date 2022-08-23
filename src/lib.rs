use log::info;
pub mod brightness;
pub mod cli_parser;
pub mod screens;
pub mod sys_tray;
pub fn init() {
    let log_lev = cli_parser::get_verbosity();
    simple_logger::init_with_level(log_lev).unwrap();
    info!("Starting with log_lev:- {:?}", log_lev);
}
