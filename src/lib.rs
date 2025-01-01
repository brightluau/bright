#[inline(always)]
pub fn version() -> &'static str {
	env!("CARGO_PKG_VERSION")
}

#[macro_export]
macro_rules! directory {
	() => {{
		use directories::UserDirs;

		// TODO: is this the best place for this?
		UserDirs::new()
			.context("Could not find home directory")
			.unwrap()
			.home_dir()
			.join(".bright")
	}};
}
