use std::{fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use include_dir::{include_dir, Dir};
use owo_colors::{colors::BrightBlack, OwoColorize};

use crate::symbols::Symbols::Success;

use super::CliCommand;

// very confusingly, include_dir!() is based at the crate's project root, which is a behavior difference from
// include_str!. *dizzy*
static TYPEDEFS: Dir<'_> = include_dir!("./include/types");

/// Installs the Luau type definitions and updates your settings files
#[derive(Parser)]
pub struct Command {
	/// Forces the reinstallation even if the type definitions don't need updating
	#[arg(short, long)]
	force: bool,
}

impl CliCommand for Command {
	fn run(self) -> Result<()> {
		if !self.force && !typedefs_need_update()? {
			println!(
				"{} Your typedefs are up to date! {}",
				Success,
				"Want to reinstall them? Rerun with --force."
					.fg::<BrightBlack>()
					.italic()
			);

			return Ok(());
		}

		match install_typedefs() {
			Err(e) => {
				bail!("Could not install typedefs: {e}")
			}
			_ => {}
		}

		println!(
			"{} Typedefs installed at `{}`",
			Success,
			typedefs_directory().display()
		);

		Ok(())
	}
}

pub fn typedefs_need_update() -> Result<bool> {
	let typedefs_directory = typedefs_directory();

	// have the typedefs for this version been installed yet?
	if !typedefs_directory.try_exists()? {
		return Ok(true);
	}

	Ok(false)
}

fn install_typedefs() -> Result<()> {
	let directory = &typedefs_directory();

	fs::create_dir_all(directory).context("Could not create Bright home directory")?;

	for entry in TYPEDEFS.entries() {
		let file = entry.as_file().unwrap();
		let name = file
			.path()
			.file_name()
			.unwrap()
			.to_string_lossy()
			.to_string();

		fs::write(directory.join(name), file.contents()).context("Could not write typedef file")?;
	}

	Ok(())
}

fn typedefs_directory() -> PathBuf {
	bright::directory!()
		.join("typedefs")
		.join(bright::version())
}
