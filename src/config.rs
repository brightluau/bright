use std::{fs, sync::OnceLock};

use mlua::{Lua, LuaSerdeExt};
use serde::Deserialize;
use toml::Table;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
	source: String,
	output: String,
	transformers: Vec<String>,
	rules: Option<Table>,
}

static INSTANCE: OnceLock<Config> = OnceLock::new();

impl Config {
	pub fn global() -> &'static Config {
		INSTANCE.get().expect("Config is not initialized")
	}

	pub fn load() -> Result<(), toml::de::Error> {
		let config = match fs::read_to_string("bright.toml") {
			Ok(contents) => toml::from_str(&contents),
			_ => Ok(Config::default()),
		}
		.unwrap();

		INSTANCE.set(config).unwrap();

		Ok(())
	}

	pub fn transformers(&self) -> &Vec<String> {
		&self.transformers
	}

	pub fn transformer_rules<'lua>(
		&self,
		name: &String,
		lua: &'lua Lua,
	) -> Result<mlua::Value<'lua>, mlua::Error> {
		let empty_table = &Table::default();

		let rules = self.rules.as_ref().unwrap_or(empty_table);

		lua.to_value(&rules.get(name))
	}

	pub fn source(&self) -> &String {
		&self.source
	}

	pub fn output(&self) -> &String {
		&self.output
	}
}

impl Default for Config {
	fn default() -> Config {
		Config {
			source: "src/".to_string(),
			output: "output/".to_string(),
			transformers: vec![],
			rules: None,
		}
	}
}
