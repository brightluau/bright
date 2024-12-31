use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use color_eyre::Result;

use crate::{
	runtime::Runtime,
	symbols::{ERROR, IMPORTANT, SUCCESS, WARNING},
};

use super::{install::typedefs_need_update, CliCommand};

/// Runs the configured transformers over source code
#[derive(Default, Parser)]
pub struct Command {
	/// The transformers to execute
	transformers: Option<Vec<String>>,

	/// The folder containing the files to be transformed, or an individual file
	#[arg(short, long, default_value = "src/")]
	input: PathBuf,

	/// The destination folder for the transformed files, or an individual file
	#[arg(short, long, default_value = "output/")]
	output: PathBuf,
}

impl CliCommand for Command {
	fn run(self) -> Result<ExitCode> {
		match typedefs_need_update() {
			Ok(true) => println!(
				"{} Your typedefs need updating! Run `{} install` to update them.",
				*IMPORTANT,
				clap::crate_name!()
			),
			Err(e) => eprintln!("{} Could not check if typedefs needed updating: {}", *WARNING, e),
			_ => {},
		};

		let runtime = Runtime::new()?;

		let transformers = match self.transformers {
			Some(transformers) => transformers,
			_ => vec![],
		};

		if transformers.is_empty() {
			println!("Nothing to do.");
			return Ok(ExitCode::FAILURE);
		}

		for transformer in &transformers {
			let result =
				runtime.run_transformer(&PathBuf::from("./tests/transformers").join(transformer));

			match result {
				Ok(()) => println!("{} transformer {} ran successfully", *SUCCESS, transformer),
				Err(e) => eprintln!("{} transformer {} failed:\n\n{}", *ERROR, transformer, e),
			}
		}

		Ok(ExitCode::SUCCESS)
	}
}
