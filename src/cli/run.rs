use std::{fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use color_eyre::Result;
use owo_colors::{colors::BrightBlack, OwoColorize};

use crate::{
	config::Config,
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

struct Transformer(String, PathBuf);

impl CliCommand for Command {
	fn run(self, config: &Config) -> Result<ExitCode> {
		match typedefs_need_update() {
			Ok(true) => println!(
				"{} Your typedefs need updating! Run `{} install` to update them.",
				*IMPORTANT,
				clap::crate_name!()
			),
			Err(e) => eprintln!(
				"{} Could not check if typedefs needed updating: {}",
				*WARNING, e
			),
			_ => {}
		};

		let runtime = Runtime::new()?;

		let transformers = match &self.transformers {
			Some(transformers) => transformers,
			_ => config.get_transformers(),
		};

		if transformers.is_empty() {
			println!("{} Nothing to do. {}", *ERROR, "(Have you configured any transformers in bright.toml?)".fg::<BrightBlack>());
			return Ok(ExitCode::FAILURE);
		}

		let mut transformer_stack: Vec<Transformer> = vec![];

		for transformer in transformers {
			match find_transformer(transformer)? {
				Some(path) => {
					transformer_stack.push(Transformer(transformer.to_string(), path));
				},

				None => {
					eprintln!("{} Could not find transformer `{}`", *ERROR, transformer);
					return Ok(ExitCode::FAILURE);
				},
			}
		}

		for transformer in transformer_stack {
			let result = runtime.run_transformer(
				&transformer.0,
				&transformer.1,
				&config,
			);

			match result {
				Ok(()) => println!("{} Transformer {} applied", *SUCCESS, transformer.0),
				Err(e) => eprintln!("{} Transformer {} failed:\n{}", *ERROR, transformer.0, e),
			}
		}

		Ok(ExitCode::SUCCESS)
	}
}

fn find_transformer(name: &String) -> Result<Option<PathBuf>> {
	// try to find it in the bright folder
	let path = PathBuf::from("./bright/transformers").join(name.to_string() + ".luau");
	let meta = fs::metadata(&path);

	match meta {
		Ok(info) => {
			if info.is_file() {
				return Ok(Some(path));
			}
		},
		_ => {}
	}

	// try to see if it's a literal file path
	let path = PathBuf::from(name);
	let meta = fs::metadata(&path);

	match meta {
		Ok(info) => {
			if info.is_file() {
				return Ok(Some(path));
			}
		},
		_ => {}
	}

	// is it missing the .luau extension?
	let path = PathBuf::from(name.to_string() + ".luau");
	let meta = fs::metadata(&path);

	match meta {
		Ok(info) => {
			if info.is_file() {
				return Ok(Some(path));
			}
		},
		_ => {}
	}

	// no idea what this is
	Ok(None)
}
