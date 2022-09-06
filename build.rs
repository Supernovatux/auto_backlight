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
    generate_to(
        Bash,
        &mut cmd,         // We need to specify what generator to use
        "auto-backlight", // We need to specify the bin name manually
        outdir,           // We need to specify where to write to
    )?;
    generate_to(
        Zsh,
        &mut cmd,         // We need to specify what generator to use
        "auto-backlight", // We need to specify the bin name manually
        outdir,           // We need to specify where to write to
    )?;
    generate_to(
        Fish,
        &mut cmd,         // We need to specify what generator to use
        "auto-backlight", // We need to specify the bin name manually
        outdir,           // We need to specify where to write to
    )?;

    Ok(())
}
