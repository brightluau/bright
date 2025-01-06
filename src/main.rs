use std::process::exit;

use anyhow::Result;
use clap::Parser;
use config::Config;
use formatting::Symbols::Error;

pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod formatting;
pub(crate) mod runtime;

use self::cli::Command;

/// Bright, a scriptable tool for transforming and transpiling Luau code
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Option<Command>,
}

fn main() -> Result<()> {
	if let Err(e) = Config::load() {
		eprintln!("{Error} Could not parse config:\n{e}");
		exit(1);
	}

	let cli = Cli::parse();

	if let Err(e) = cli.command.unwrap_or_default().run() {
		eprintln!("{Error} {e:?}");
		exit(1);
	};

	Ok(())
}
