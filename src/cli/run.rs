use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;

use crate::{
	runtime::Runtime,
	symbols::{ERROR, INFO, SUCCESS},
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
	fn run(self) -> Result<()> {
		match typedefs_need_update() {
			Ok(true) => println!(
				"{} Your typedefs need updating! Run `{} install` to update them.",
				*INFO,
				clap::crate_name!()
			),
			_ => {}
		};

		let runtime = Runtime::new()?;

		let transformers = match self.transformers {
			Some(transformers) => transformers,
			_ => vec![],
		};

		for transformer in &transformers {
			let result = runtime.run_transformer(&PathBuf::from("./tests/transformers").join(transformer));

			match result {
				Ok(()) => println!("{} transformer {} ran successfully", *SUCCESS, transformer),
				Err(e) => eprintln!(
					"{} transformer {} failed:\n\n{}",
					*ERROR,
					transformer,
					e
				),
			}
		}

		Ok(())
	}
}
