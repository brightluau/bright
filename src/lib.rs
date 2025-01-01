use std::path::PathBuf;

use anyhow::Context;
use directories::UserDirs;

#[inline(always)]
pub fn version() -> &'static str {
	env!("CARGO_PKG_VERSION")
}

#[inline(always)]
pub fn directory() -> PathBuf {
	// TODO: is this the best place for this?
	UserDirs::new()
		.context("Could not find home directory")
		.unwrap()
		.home_dir()
		.join(".bright")
}
