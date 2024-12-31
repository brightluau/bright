use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;

use super::CliCommand;

/// Runs a singular transformer for testing purposes
#[derive(Parser)]
pub struct Command {
	/// The transformer to test
	rule: String,

	/// The source file to run the transformer on
	source: PathBuf,

	/// The output file to write the transformed code to
	output: PathBuf,
}

impl CliCommand for Command {
	fn run(self) -> Result<()> {
		Ok(())
	}
}
