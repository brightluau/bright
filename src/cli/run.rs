use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use full_moon::{
	ast::Ast,
	Error::{AstError, TokenizerError},
};
use owo_colors::{
	colors::{BrightBlack, Yellow},
	OwoColorize,
};
use walkdir::WalkDir;

use crate::{
	config::Config,
	runtime::{Runtime, Transformer},
	symbols::Symbols::{Error, Important, Success, Warning},
};

use super::{install::typedefs_need_update, CliCommand};

/// Runs the configured transformers over source code
#[derive(Parser)]
pub struct Command {
	/// The transformers to execute
	transformers: Option<Vec<String>>,

	/// The folder containing the files to be transformed, or an individual file
	#[arg(short, long, default_value = Config::global().source(), value_parser)]
	input: PathBuf,

	/// The destination folder for the transformed files, or an individual file
	#[arg(short, long, default_value = Config::global().output(), value_parser)]
	output: PathBuf,
}

impl Default for Command {
	fn default() -> Self {
		Self {
			input: PathBuf::from(Config::global().source()),
			output: PathBuf::from(Config::global().output()),

			transformers: Default::default(),
		}
	}
}

impl CliCommand for Command {
	fn run(self) -> Result<()> {
		let config = Config::global();

		match typedefs_need_update() {
			Ok(true) => println!(
				"{Important} Your typedefs need updating! Run `{}` to update them.",
				format!("{} install", clap::crate_name!())
					.fg::<Yellow>()
					.bold()
			),
			Err(e) => eprintln!(
				"{Warning} Could not check if typedefs needed updating: {}",
				e
			),
			_ => {}
		};

		if !self.input.try_exists()? {
			bail!("Source path `{}` does not exist.", self.input.display())
		}

		let runtime = Runtime::new()?;

		let transformers = match &self.transformers {
			Some(transformers) => transformers,
			_ => config.transformers(),
		};

		if transformers.is_empty() {
			bail!(
				"No transformers to run. {}",
				"Have you configured any transformers in bright.toml?"
					.fg::<BrightBlack>()
					.italic()
			)
		}

		// locate all transformers and compile them

		let mut transformer_stack: Vec<Transformer> = vec![];

		for transformer_name in transformers {
			match find_transformer(transformer_name)? {
				Some(path) => {
					let transformer = runtime
						.compile_transformer(&transformer_name, &path)
						.context(format!(
							"Could not compile transformer `{transformer_name}`"
						))?;

					transformer_stack.push(transformer);
				}

				None => {
					bail!("Could not find transformer `{transformer_name}`")
				}
			}
		}

		// load source files

		let mut sources: HashMap<PathBuf, Ast> = HashMap::new();

		// is this a singular source file?
		if self.input.metadata()?.is_file() {
			let path = &self.input;

			let source = fs::read_to_string(path)?;

			let ast = match full_moon::parse(&source) {
				Ok(ast) => ast,
				Err(errors) => {
					bail!(
						"Failed to parse `{}`:\n{}",
						self.input.display(),
						format_full_moon_errors(errors)
					);
				}
			};

			sources.insert(path.strip_prefix(&self.input)?.to_path_buf(), ast);
		} else {
			for path in WalkDir::new(&self.input) {
				let path = path.unwrap();

				// ignore non-files
				if !path.metadata()?.is_file() {
					continue;
				}

				match path.path().extension() {
					Some(extension) => {
						if extension != "luau" && extension != "lua" {
							continue;
						}
					}
					_ => continue,
				}

				let source = fs::read_to_string(path.path())?;

				let ast = match full_moon::parse(&source) {
					Ok(ast) => ast,
					Err(errors) => {
						// don't end the whole operation if one source file doesn't parse, just skip it

						eprintln!(
							"{Error} Failed to parse `{}`:\n{}",
							path.path().display(),
							format_full_moon_errors(errors)
						);

						continue;
					}
				};

				sources.insert(path.path().strip_prefix(&self.input)?.to_path_buf(), ast);
			}
		}

		if sources.len() == 0 {
			bail!(
				"No sources to transform. {}",
				"Do you need to change your source directory? Set `source` in bright.toml or pass --input."
					.fg::<BrightBlack>().italic()
			)
		}

		// transform source code

		for transformer in transformer_stack {
			let result = runtime.run_transformer(&transformer, &config);

			match result {
				Ok(()) => println!("{Success} Transformer `{}` applied", transformer.name),
				Err(e) => eprintln!("{Error} Transformer `{}` failed:\n{}", transformer.name, e),
			}
		}

		// write to output

		fs::create_dir_all(&self.output)?;

		for (path, ast) in sources {
			let target = self.output.join(path);

			fs::create_dir_all(target.parent().unwrap())?;

			fs::write(target, ast.to_string())?;
		}

		Ok(())
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
		}
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
		}
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
		}
		_ => {}
	}

	// no idea what this is
	Ok(None)
}

fn format_full_moon_errors(errors: Vec<full_moon::Error>) -> String {
	errors
		.iter()
		.map(|error| match error {
			AstError(e) => format!("{e}"),
			TokenizerError(e) => format!("{e}"),
		})
		.fold(String::new(), |a, b| a + &b + "\n")
		.trim()
		.to_string()
}
