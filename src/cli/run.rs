use std::{collections::HashMap, fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use color_eyre::Result;
use full_moon::{
	ast::Ast,
	Error::{AstError, TokenizerError},
};
use owo_colors::{colors::BrightBlack, OwoColorize};
use walkdir::WalkDir;

use crate::{
	config::Config,
	runtime::{Runtime, Transformer},
	symbols::{ERROR, IMPORTANT, SUCCESS, WARNING},
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
	fn run(self) -> Result<ExitCode> {
		let config = Config::global();

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

		if !self.input.try_exists()? {
			eprintln!(
				"{} Source path `{}` does not exist.",
				*ERROR,
				self.input.display()
			);
			return Ok(ExitCode::FAILURE);
		}

		let runtime = Runtime::new()?;

		let transformers = match &self.transformers {
			Some(transformers) => transformers,
			_ => config.transformers(),
		};

		if transformers.is_empty() {
			eprintln!(
				"{} Nothing to do. {}",
				*ERROR,
				"(Have you configured any transformers in bright.toml?)".fg::<BrightBlack>()
			);
			return Ok(ExitCode::FAILURE);
		}

		// locate all transformers and compile them

		let mut transformer_stack: Vec<Transformer> = vec![];

		for transformer_name in transformers {
			match find_transformer(transformer_name)? {
				Some(path) => {
					let transformer = match runtime.compile_transformer(transformer_name, &path) {
						Ok(transformer) => transformer,
						Err(e) => {
							eprintln!(
								"{} Could not compile transformer `{}`:\n{}",
								*ERROR, transformer_name, e
							);
							return Ok(ExitCode::FAILURE);
						}
					};

					transformer_stack.push(transformer);
				}

				None => {
					eprintln!(
						"{} Could not find transformer `{}`",
						*ERROR, transformer_name
					);
					return Ok(ExitCode::FAILURE);
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
					eprintln!("{} Failed to parse `{}`:", *ERROR, self.input.display());

					for error in errors {
						match error {
							AstError(err) => eprintln!("{}", err),
							TokenizerError(err) => eprintln!("{}", err),
						}
					}

					return Ok(ExitCode::FAILURE);
				}
			};

			sources.insert(path.to_path_buf(), ast);
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
						eprintln!("{} Failed to parse `{}`:", *ERROR, path.path().display());

						for error in errors {
							match error {
								AstError(err) => eprintln!("{}", err),
								TokenizerError(err) => eprintln!("{}", err),
							}
						}

						continue;
					}
				};

				sources.insert(path.path().to_path_buf(), ast);
			}
		}

		if sources.len() == 0 {
			eprintln!(
				"{} Nothing to do. {}",
				*ERROR,
				"(Do you need to change your source directory? Set `source` in bright.toml or pass --input)".fg::<BrightBlack>()
			);
			return Ok(ExitCode::FAILURE);
		}

		// transform source code

		for transformer in transformer_stack {
			let result = runtime.run_transformer(&transformer, &config);

			match result {
				Ok(()) => println!("{} Transformer `{}` applied", *SUCCESS, transformer.name),
				Err(e) => eprintln!(
					"{} Transformer `{}` failed:\n{}",
					*ERROR, transformer.name, e
				),
			}
		}

		// write to output

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
