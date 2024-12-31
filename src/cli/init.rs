use std::process::ExitCode;

use clap::Parser;
use color_eyre::Result;

use crate::config::Config;

use super::CliCommand;

/// Initializes the current folder with a Bright setup
#[derive(Parser)]
pub struct Command {}

impl CliCommand for Command {
	fn run(self, _config: &Config) -> Result<ExitCode> {
		Ok(ExitCode::SUCCESS)
	}
}
