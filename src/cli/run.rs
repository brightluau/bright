use std::{
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
	sync::mpsc,
	time::Duration,
};

use anyhow::{bail, Context, Result};
use clap::Parser;
use console::style;
use full_moon::{
	ast::Ast,
	Error::{AstError, TokenizerError},
};
use notify::{EventKind, RecursiveMode, Watcher};
use walkdir::WalkDir;

use crate::{
	config::Config,
	formatting::{
		hint,
		Symbols::{Error, Important, Success, Warning},
	},
	runtime::{Runtime, Transformer},
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

	/// Watch the file system for changes and re-run the transformers
	#[arg(short, long)]
	watch: bool,
}

impl Default for Command {
	fn default() -> Self {
		Self {
			input: PathBuf::from(Config::global().source()),
			output: PathBuf::from(Config::global().output()),

			transformers: Default::default(),
			watch: Default::default(),
		}
	}
}

impl CliCommand for Command {
	fn run(self) -> Result<()> {
		let config = Config::global();

		match typedefs_need_update() {
			Ok(true) => println!(
				"{Important} Your typedefs need updating! Run `{}` to update them.",
				style(format!("{} install", clap::crate_name!()))
					.yellow()
					.bold()
			),
			Err(e) => eprintln!("{Warning} Could not check if typedefs needed updating: {e}"),
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
				hint("Have you configured any transformers in bright.toml?")
			)
		}

		// locate all transformers and compile them

		let mut transformer_stack: Vec<Transformer> = vec![];

		for transformer_name in transformers {
			match find_transformer(transformer_name)? {
				Some(path) => {
					let transformer = runtime
						.compile_transformer(transformer_name, &path)
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

		let mut sources = build_sources_map(&self.input, self.watch)?;

		if sources.is_empty() {
			bail!(
				"No sources to transform. {}",
				hint("Do you need to change your source directory? Set `source` in bright.toml or pass --input.")
			)
		}

		// transform source code

		run_transformers(&transformer_stack, &mut sources, &runtime, config);

		// write to output

		write_sources(&sources, &self.output)?;

		if self.watch {
			// if we're watching, then we set up a filesystem watcher and wait for changes after our first pass
			let (tx, rx) = mpsc::channel();

			let mut watcher = notify::PollWatcher::new(
				tx,
				notify::Config::default()
					.with_poll_interval(Duration::from_secs(1))
					.with_compare_contents(true),
			)?;

			watcher.watch(&self.input, RecursiveMode::Recursive)?;

			println!(
				"{Success} Now watching {} for changes",
				self.input.display()
			);

			for res in rx {
				let event = match res {
					Ok(event) => {
						let mut do_we_care = false;

						for path in &event.paths {
							match path.extension() {
								Some(extension) => {
									if extension != "luau" && extension != "lua" {
										continue;
									}
								}
								_ => continue,
							}

							// it's matching a lua(u) file, we care about this one
							do_we_care = true;
							break;
						}

						if do_we_care {
							Some(event)
						} else {
							None
						}
					}
					Err(e) => bail!("Encountered an error during watch: {e}"),
				};

				if let Some(event) = event {
					println!();

					// get the paths to luau files
					let mut paths: Vec<PathBuf> = vec![];

					for path in event.paths {
						match path.extension() {
							Some(extension) => {
								if extension == "luau" || extension == "lua" {
									paths.push(path);
								}
							}
							_ => continue,
						}
					}

					match event.kind {
						EventKind::Create(..) | EventKind::Modify(..) | EventKind::Any => {
							let mut sources =
								build_sources_map_from_paths(paths, &self.input, true)?;

							run_transformers(&transformer_stack, &mut sources, &runtime, config);

							write_sources(&sources, &self.output)?;
						}
						EventKind::Remove(..) => {
							for path in paths {
								let output_path =
									&self.output.join(path.strip_prefix(&self.input)?);
								fs::remove_file(output_path)?;

								println!("{Important} `{}` removed from output", path.display());
							}
						}
						_ => {}
					}
				}
			}
		}

		Ok(())
	}
}

fn find_transformer(name: &String) -> Result<Option<PathBuf>> {
	// try to find it in the bright folder
	let path = PathBuf::from_iter(vec![
		".",
		"bright",
		"transformers",
		&(name.to_string() + ".luau"),
	]);
	let meta = fs::metadata(&path);

	if let Ok(info) = meta {
		if info.is_file() {
			return Ok(Some(path));
		}
	}

	// try to see if it's a literal file path
	let path = PathBuf::from(name);
	let meta = fs::metadata(&path);

	if let Ok(info) = meta {
		if info.is_file() {
			return Ok(Some(path));
		}
	}

	// is it missing the .luau extension?
	let path = PathBuf::from(name.to_string() + ".luau");
	let meta = fs::metadata(&path);

	if let Ok(info) = meta {
		if info.is_file() {
			return Ok(Some(path));
		}
	}

	// no idea what this is
	Ok(None)
}

fn build_sources_map(input: &Path, watch_mode: bool) -> Result<HashMap<PathBuf, Ast>> {
	// is this a singular source file?
	if input.metadata()?.is_file() {
		Ok(build_sources_map_from_paths(
			vec![input.to_path_buf()],
			input,
			watch_mode,
		)?)
	} else {
		let mut paths: Vec<PathBuf> = vec![];

		for path in WalkDir::new(input) {
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

			paths.push(path.path().to_path_buf());
		}

		Ok(build_sources_map_from_paths(paths, input, watch_mode)?)
	}
}

fn build_sources_map_from_paths(
	paths: Vec<PathBuf>,
	base: &Path,
	watch_mode: bool,
) -> Result<HashMap<PathBuf, Ast>> {
	let mut sources: HashMap<PathBuf, Ast> = HashMap::new();

	for path in paths {
		let source = fs::read_to_string(&path)?;

		let ast = match full_moon::parse(&source) {
			Ok(ast) => ast,
			Err(errors) => {
				if !watch_mode {
					// even with a bunch of files, if we're not running in watch mode we shouldn't do anything at all;
					// there is zero benefit to only converting *most* of the files when you need *all* of them for
					// a working program or game (plus we might produce non-functioning code)
					bail!(
						"{Error} Failed to parse `{}`:\n{}",
						path.display(),
						format_full_moon_errors(errors)
					)
				}

				eprintln!(
					"{Error} Failed to parse `{}`:\n{}",
					path.display(),
					format_full_moon_errors(errors)
				);

				continue;
			}
		};

		sources.insert(path.strip_prefix(base)?.to_path_buf(), ast);
	}

	Ok(sources)
}

fn run_transformers(
	stack: &Vec<Transformer>,
	sources: &mut HashMap<PathBuf, Ast>,
	runtime: &Runtime,
	config: &Config,
) {
	for source in sources {
		for transformer in stack {
			let result = runtime.run_transformer(transformer, config);

			match result {
				Ok(()) => println!(
					"{Success} Transformer `{}` applied to {}",
					transformer.name,
					source.0.display()
				),
				Err(e) => eprintln!(
					"{Error} Transformer `{}` failed to apply to {}:\n{e}",
					transformer.name,
					source.0.display()
				),
			}
		}
	}
}

fn write_sources(sources: &HashMap<PathBuf, Ast>, output: &Path) -> Result<()> {
	fs::create_dir_all(output)?;

	for (path, ast) in sources {
		let target = output.join(path);

		fs::create_dir_all(target.parent().unwrap())?;

		fs::write(target, ast.to_string())?;
	}

	Ok(())
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
