use clap::Parser;
use color_eyre::Result;

use super::CliCommand;

/// Initializes the current folder with a Bright setup
#[derive(Parser)]
pub struct Command {}

impl CliCommand for Command {
    fn run(self) -> Result<()> {
        Ok(())
    }
}
