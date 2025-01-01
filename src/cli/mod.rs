use anyhow::Result;
use clap::Subcommand;

pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod run;

pub trait CliCommand {
	fn run(self) -> Result<()>;
}

#[derive(Subcommand)]
pub enum Command {
	Init(init::Command),
	Run(run::Command),
	Install(install::Command),
}

impl Command {
	pub fn run(self) -> Result<()> {
		match self {
			Self::Init(cmd) => cmd.run(),
			Self::Run(cmd) => cmd.run(),
			Self::Install(cmd) => cmd.run(),
		}
	}
}

impl Default for Command {
	fn default() -> Self {
		Self::Run(run::Command::default())
	}
}
