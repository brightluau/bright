use std::fs;

use mlua::{Lua, LuaSerdeExt};
use serde::Deserialize;
use toml::Table;

#[derive(Deserialize)]
pub struct Config {
	transformers: Vec<String>,
	rules: Option<Table>,
}

impl Config {
	pub fn load() -> Result<Config, toml::de::Error> {
		match fs::read_to_string("bright.toml") {
			Ok(contents) => toml::from_str(&contents),
			_ => Ok(Config::default()),
		}
	}

	pub fn get_transformers(&self) -> &Vec<String> {
		&self.transformers
	}

	pub fn get_transformer_rules<'lua>(
		&self,
		name: &String,
		lua: &'lua Lua,
	) -> Result<mlua::Value<'lua>, mlua::Error> {
		let empty_table = &Table::default();

		let rules = self.rules.as_ref().unwrap_or(empty_table);

		lua.to_value(&rules.get(name))
	}
}

impl Default for Config {
	fn default() -> Config {
		Config {
			transformers: vec![],
			rules: None,
		}
	}
}
