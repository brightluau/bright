use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use clap::Parser;

use crate::config::{CONFIG_DEFAULT_CONTENTS, CONFIG_FILE_NAME};

use super::CliCommand;

/// Initializes the current folder with a Bright setup
#[derive(Parser)]
pub struct Command {}

impl CliCommand for Command {
	fn run(self) -> Result<()> {
		let path = PathBuf::from(CONFIG_FILE_NAME);

		if path.exists() {
			bail!("There's already a Bright setup here! It would be quite unwise if your configuration was destroyed!")
		}

		fs::write(path, CONFIG_DEFAULT_CONTENTS)?;
		fs::create_dir_all(PathBuf::from("./bright/transformers"))?;

		Ok(())
	}
}
