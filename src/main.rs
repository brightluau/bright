use std::process::exit;

use anyhow::Result;
use clap::Parser;
use config::Config;
use symbols::Symbols::Error;

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

fn main() -> Result<()> {
	if let Err(e) = Config::load() {
		eprintln!("{} Could not parse config:\n{e}", Error);
		exit(1);
	}

	let cli = Cli::parse();

	match cli.command.unwrap_or_default().run() {
		Err(e) => {
			eprintln!("{} {e:?}", Error);
			exit(1);
		}
		_ => {}
	};

	Ok(())
}
