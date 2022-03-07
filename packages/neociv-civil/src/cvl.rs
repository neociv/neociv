use rlua::{Lua, Result};

/// Generate the bindings for the "cvl" global
pub fn init() -> Lua {
    let lua = Lua::new();

    let result: Result<()> = lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        let cvl = lua_ctx.create_table()?;

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
