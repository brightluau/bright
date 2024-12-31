use clap::Subcommand;
use color_eyre::Result;

pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod run;
pub(crate) mod test;

pub trait CliCommand {
	fn run(self) -> Result<()>;
}

#[derive(Subcommand)]
pub enum Command {
	Init(init::Command),
	Run(run::Command),
	Test(test::Command),
	Install(install::Command),
}

impl Default for Command {
	fn default() -> Self {
		Self::Run(run::Command::default())
	}
}
