use std::{fs, path::PathBuf};

use color_eyre::{eyre::Report, Result};
use mlua::{prelude::*, Function, StdLib};

mod globals;

pub struct Runtime {
    lua: Lua,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let options = LuaOptions::new().catch_rust_panics(false); // panics should result in the runtime crashing

        let lua = Lua::new_with(StdLib::ALL_SAFE, options)?;

		globals::inject_globals(&lua)?;

        lua.sandbox(true)?;

        Ok(Runtime { lua })
    }

    pub fn run_transformer(self, path: &PathBuf) -> Result<()> {
        let contents = fs::read_to_string(path).expect("transformer script does not exist");

        let script = self
            .lua
            .load(contents)
            .set_name(format!("@{}", path.display()));
        let function = script.eval::<Function>();

        match function {
            Ok(func) => Ok(func.call::<_, ()>(())?),

            // handle conversion errors differently since it's not exactly clear when this fails
            Err(mlua::Error::FromLuaConversionError { .. }) => {
                Err(Report::msg("script did not return function"))
            }

            // normal errors just get converted into eyre reports and passed up
            Err(e) => Err(Report::new(e)),
        }
    }
}
