use owo_colors::{
	colors::{Blue, Green, Red, Yellow},
	OwoColorize,
};

lazy_static::lazy_static! {
	pub static ref SUCCESS: String = '✔'.fg::<Green>().bold().to_string();
	pub static ref ERROR: String = '✖'.fg::<Red>().bold().to_string();
	pub static ref WARNING: String = '!'.fg::<Yellow>().bold().to_string();
	pub static ref IMPORTANT: String = '★'.fg::<Blue>().bold().to_string();
}
