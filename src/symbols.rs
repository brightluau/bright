use owo_colors::{
	colors::{Blue, Green, Red},
	OwoColorize,
};

lazy_static::lazy_static! {
	pub static ref SUCCESS: String = '✔'.fg::<Green>().to_string();
	pub static ref ERROR: String = '✖'.fg::<Red>().to_string();
	pub static ref INFO: String = '★'.fg::<Blue>().to_string();
}
