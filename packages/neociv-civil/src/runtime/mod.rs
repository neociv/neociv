pub mod lua;

use rlua::{Error as LuaError, Lua as RLua};
use crate::runtime::lua::NeocivLuaRuntime;

pub fn init_lua() -> Result<impl NeocivLuaRuntime, LuaError> {
    return init_lua_sdk("rlua");
}

pub fn init_lua_sdk(sdk: &str) -> Result<impl NeocivLuaRuntime, LuaError> {
    if sdk.eq("rlua") {
        return RLua::init();
    } else {
        panic!("Unknown lua sdk '{}'", sdk);
    }
}
