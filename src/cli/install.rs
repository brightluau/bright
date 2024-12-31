use std::{fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use include_dir::{include_dir, Dir};
use owo_colors::{colors::BrightBlack, OwoColorize};

use crate::symbols::SUCCESS;

use super::CliCommand;

static TYPEDEFS: Dir<'_> = include_dir!("types");

/// Installs the Luau type definitions and updates your settings files
#[derive(Parser)]
pub struct Command {
	/// Forces the reinstallation even if the type definitions don't need updating
	#[arg(short, long)]
	force: bool,
}

impl CliCommand for Command {
	fn run(self) -> Result<ExitCode> {
		if !self.force && !typedefs_need_update()? {
			println!(
				"{} Your typedefs are up to date! {}",
				*SUCCESS,
				"(Want to reinstall them? Rerun with --force)".fg::<BrightBlack>()
			);

			return Ok(ExitCode::SUCCESS);
		}

		install_typedefs().expect("could not install typedefs");

		println!(
			"{} Typedefs installed at `{}`",
			*SUCCESS,
			typedefs_directory().display()
		);

		Ok(ExitCode::SUCCESS)
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

	fs::create_dir_all(directory).context("could not create bright home directory")?;

	for entry in TYPEDEFS.entries() {
		let file = entry.as_file().unwrap();
		let name = file
			.path()
			.file_name()
			.unwrap()
			.to_string_lossy()
			.to_string();

		fs::write(directory.join(name), file.contents()).context("could not write typedef file")?;
	}

	Ok(())
}

fn typedefs_directory() -> PathBuf {
	bright::directory!()
		.join("typedefs")
		.join(bright::version())
}
