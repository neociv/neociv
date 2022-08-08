use rlua::{Lua, Result as LuaResult};

/// Generate the bindings for the "cvl" global
pub fn init() -> Lua {
    let lua = Lua::new();

    let result: LuaResult<()> = lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        let cvl = lua_ctx.create_table()?;

        // Create a key mapping registry
        cvl.set("_key_maps", lua_ctx.create_table()?)?;

        // Create an event registry
        cvl.set("_event_registry", lua_ctx.create_table()?)?;

        // "on" will be used to add items to the event registry

        // TODO: New state handling


        globals.set("cvl", cvl)?;

        return Ok(());
    });

    return match result {
        Ok(_) => lua,
        Err(ex) => panic!("{:?}", ex),
    };
}

pub fn load_file(lua: &Lua, filepath: String) -> Result<(),()> {
    return Err(());
}

pub fn load_lua_str(lua: &Lua, content: String) -> Result<(),()> {
    return Err(());
}

pub fn load_cvl_str(lua: &Lua, content: String) -> Result<(),()> {
    return Err(());
}
