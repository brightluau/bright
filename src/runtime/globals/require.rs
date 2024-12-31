use mlua::prelude::*;

// require is disabled in bright as it is purely used for LSPs to load the type definitions

pub fn create(lua: &Lua) -> LuaResult<LuaValue> {
	lua.load("return nil")
		.set_name("require")
		.into_function()?
		.into_lua(lua)
}
