use std::{fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use color_eyre::Result;

use crate::{
	config::{CONFIG_DEFAULT_CONTENTS, CONFIG_FILE_NAME},
	symbols::ERROR,
};

use super::CliCommand;

/// Initializes the current folder with a Bright setup
#[derive(Parser)]
pub struct Command {}

impl CliCommand for Command {
	fn run(self) -> Result<ExitCode> {
		let path = PathBuf::from(CONFIG_FILE_NAME);

		if path.exists() {
			eprintln!("{} There's already a Bright setup here! It would be quite unwise if your configuration was destroyed!", *ERROR);
			return Ok(ExitCode::FAILURE);
		}

		fs::write(path, CONFIG_DEFAULT_CONTENTS)?;
		fs::create_dir_all(PathBuf::from("./bright/transformers"))?;

		Ok(ExitCode::SUCCESS)
	}
}
