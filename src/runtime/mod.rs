use std::{fs, path::PathBuf};

use color_eyre::{eyre::Report, Result};
use mlua::{prelude::*, Function, StdLib};

use crate::config::Config;

mod globals;

pub struct Runtime {
	lua: Lua,
}

pub struct Transformer<'a> {
	pub name: String,
	function: Function<'a>,
}

impl Runtime {
	pub fn new() -> Result<Self> {
		let options = LuaOptions::new().catch_rust_panics(false); // panics should result in the runtime crashing

		let lua = Lua::new_with(StdLib::ALL_SAFE, options)?;

		globals::inject_globals(&lua)?;

		lua.sandbox(true)?;

		Ok(Self { lua })
	}

	pub fn compile_transformer(&self, name: &str, path: &PathBuf) -> Result<Transformer> {
		let contents = fs::read_to_string(&path).expect("transformer script does not exist");

		let script = self
			.lua
			.load(contents)
			.set_name(format!("@{}", path.display()));

		let function = script.eval::<Function>();

		match function {
			Ok(func) => Ok(Transformer {
				name: name.to_string(),
				function: func,
			}),

			// handle conversion errors differently since it's not exactly clear when this fails
			Err(mlua::Error::FromLuaConversionError { .. }) => {
				Err(Report::msg("script did not return function"))
			}

			// normal errors just get converted into eyre reports and passed up
			Err(e) => Err(Report::new(e)),
		}
	}

	pub fn run_transformer(&self, transformer: &Transformer, config: &Config) -> Result<()> {
		let transformer_rules = config.transformer_rules(&transformer.name, &self.lua)?;

		Ok(transformer.function.call::<_, ()>(&(transformer_rules))?)
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_matrix;

	use super::*;

	fn project_root() -> PathBuf {
		PathBuf::from(env!("CARGO_MANIFEST_DIR"))
	}

	#[test_matrix("blank")]
	fn transformer(name: &'static str) {
		let runtime = Runtime::new().expect("could not create runtime");

		let transformer_path = &project_root()
			.join("tests/transformers")
			.join(name.to_string() + ".luau");

		let transformer = runtime
			.compile_transformer(&name.to_string(), transformer_path)
			.expect("could not compile transformer");

		runtime
			.run_transformer(&transformer, &Config::default())
			.expect("could not execute transformer");
	}
}
