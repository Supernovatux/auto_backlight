use clap::*;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
};
use std::io::Error;

include!("src/cli_parser.rs");

fn main() -> Result<(), Error> {
    let outdir = "completions";
    let mut cmd = Cli::into_app();
    generate_to(Bash, &mut cmd, "auto-backlight", outdir)?;
    generate_to(Zsh, &mut cmd, "auto-backlight", outdir)?;
    generate_to(Fish, &mut cmd, "auto-backlight", outdir)?;

    Ok(())
}
