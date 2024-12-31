use std::process::ExitCode;

use clap::Parser;
use color_eyre::Result;

pub(crate) mod cli;
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

	Ok(cli.command.unwrap_or_default().run()?)
}
