use std::fmt;

use console::{style, StyledObject};

pub enum Symbols {
	Success,
	Error,
	Warning,
	Important,
}

impl fmt::Display for Symbols {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let symbol = match self {
			Self::Success => style('✔').green().bold(),
			Self::Error => style('✖').red().bold(),
			Self::Warning => style('!').yellow().bold(),
			Self::Important => style('★').blue().bold(),
		};

		write!(f, "{}", symbol)
	}
}

pub fn hint(text: &str) -> StyledObject<&str> {
	style(text).black().bright().italic()
}
