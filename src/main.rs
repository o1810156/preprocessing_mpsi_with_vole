use anyhow::{Context, Result};
use clap::Parser;
use preprocessing_mpsi_with_vole::cli_utils::PrePSIArgs;
use preprocessing_mpsi_with_vole::preprocessed::psi::run;

fn main() -> Result<()> {
    let args = PrePSIArgs::parse();

    println!("{:?}", args);

    run(args).with_context(|| "Failed to run the protocol.")?;

    Ok(())
}
