use std::process::ExitCode;

use clap::Parser;
use color_eyre::Result;
use config::Config;
use symbols::ERROR;

pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod runtime;
pub(crate) mod symbols;

use self::cli::Command;

/// Bright, a scriptable tool for transforming and transpiling Luau code
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Option<Command>,
}

fn main() -> Result<ExitCode> {
	color_eyre::install()?;

	let cli = Cli::parse();
	let config = match Config::load() {
		Ok(config) => config,
		Err(e) => {
			eprintln!("{} Could not parse config:\n{}", *ERROR, e);
			return Ok(ExitCode::FAILURE);
		}
	};

	Ok(cli.command.unwrap_or_default().run(&config)?)
}
