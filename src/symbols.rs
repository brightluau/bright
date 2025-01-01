use std::fmt;

use owo_colors::{
	colors::{Blue, Green, Red, Yellow},
	OwoColorize,
};

pub enum Symbols {
	Success,
	Error,
	Warning,
	Important,
}

impl Symbols {
	fn symbol(&self) -> String {
		match self {
			Self::Success => '✔'.fg::<Green>().bold().to_string(),
			Self::Error => '✖'.fg::<Red>().bold().to_string(),
			Self::Warning => '!'.fg::<Yellow>().bold().to_string(),
			Self::Important => '★'.fg::<Blue>().bold().to_string(),
		}
	}
}

impl fmt::Display for Symbols {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.symbol())
	}
}
