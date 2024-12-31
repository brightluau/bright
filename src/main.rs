use clap::Parser;
use cli::CliCommand;
use color_eyre::Result;

pub(crate) mod cli;
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
    color_eyre::install()?;
    colog::init();

    let cli = Cli::parse();

    match cli.command.unwrap_or_default() {
        Command::Init(cmd) => cmd.run()?,
        Command::Run(cmd) => cmd.run()?,
        Command::Test(cmd) => cmd.run()?,
        Command::Install(cmd) => cmd.run()?,
    }

    Ok(())
}
