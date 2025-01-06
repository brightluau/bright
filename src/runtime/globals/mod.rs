use mlua::prelude::*;

pub(super) mod require;

pub enum Globals {
	Require,
}

impl Globals {
	pub const ALL: &'static [Self] = &[Self::Require];

	pub fn create<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
		match self {
			Self::Require => require::create(lua),
		}
	}

	pub fn name(&self) -> &'static str {
		match self {
			Self::Require => "require",
		}
	}
}

pub fn inject_globals(lua: &Lua) -> LuaResult<()> {
	let global_table = lua.globals();

	for global in Globals::ALL {
		global_table.set(global.name(), global.create(lua)?)?;
	}

	Ok(())
}
