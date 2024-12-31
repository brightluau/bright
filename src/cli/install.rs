use clap::Parser;
use color_eyre::Result;
use include_dir::{include_dir, Dir};
use log::info;

use super::CliCommand;

static TYPEDEFS: Dir<'_> = include_dir!("types");

/// Installs the Luau type definitions and updates your settings files
#[derive(Parser)]
pub struct Command {}

impl CliCommand for Command {
    fn run(self) -> Result<()> {
        info!("installing typedefs v{}", bright::version());

        Ok(())
    }
}

pub fn typedefs_need_update() -> Result<bool> {
    Ok(false)
}
