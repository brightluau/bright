use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;

use crate::{runtime::Runtime, symbols::{ERROR, SUCCESS}};

use super::CliCommand;

/// Runs the configured transformers over source code
#[derive(Default, Parser)]
pub struct Command {
	/// The source folder to run the transformers on
	#[arg(default_value = "src/")]
	source: PathBuf,
}

impl CliCommand for Command {
	fn run(self) -> Result<()> {
		let runtime = Runtime::new()?;

		let result = runtime.run_transformer(&self.source);

		match result {
			Ok(()) => println!("{} transformer ran successfully", *SUCCESS),
			Err(e) => eprintln!(
				"{} transformer {} failed:\n\n{}",
				*ERROR,
				self.source.display(),
				e
			),
		}

		Ok(())
	}
}
