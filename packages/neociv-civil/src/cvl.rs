use indoc::formatdoc;
use std::path::Path;
use std::result::Result;

use rlua::{Error as LuaError, FromLuaMulti, Lua, Result as LuaResult};

static FENNEL_FILE: &'static str = include_str!("fennel.lua");

/// Generate the bindings for the "cvl" global
pub fn init() -> Result<Lua, LuaError> {
    let lua = Lua::new();

    let result: LuaResult<()> = lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        let cvl = lua_ctx.create_table()?;

        // Create a key mapping registry
        cvl.set("_key_maps", lua_ctx.create_table()?)?;

        // Create an event registry
        cvl.set("_event_registry", lua_ctx.create_table()?)?;

        // "on" will be used to add items to the event registry
        let on = lua_ctx.create_function(|_, name: String| {
            println!("Hello, {}!", name);
            return Ok(());
        })?;

        cvl.set("on", on)?;

        globals.set("cvl", cvl)?;

        // Add Fennel
        lua_ctx.load(FENNEL_FILE).exec()?;

        // TODO: Get a reference to the fennel compiler function

        return Ok(());
    });

    return match result {
        Ok(_) => Ok(lua),
        Err(ex) => Err(ex),
    };
}

pub fn compile_cvl(lua: &Lua, content: String) -> Result<String, LuaError> {
    let block = formatdoc! {"
            local fnl_compiler = require(\"fennel.compiler\") 
            return fnl_compiler[\"compile-string\"]([[
                {content}
            ]])
        ", content = content.as_str()};

    return eval(lua, block);
}

/// Load a file into the lua context, appropriately deciding whether to parse it as cvl (fennel) or lua.
pub fn load_file(lua: &Lua, filepath: String) -> Result<&Lua, ()> {
    return Err(());
}

/// Load a string containing lua into the context only caring if successfully parsed or not.
pub fn load_lua_str(lua: &Lua, content: String) -> Result<&Lua, LuaError> {
    lua.context(move |lua_ctx| {
        return lua_ctx.load(r#content.as_str()).exec().unwrap();
    });
    return Ok(lua);
}

/// Load a string containing cvl (fennel) into the context by pre-parsing it.
pub fn load_cvl_str(lua: &Lua, content: String) -> Result<&Lua, LuaError> {
    return load_lua_str(lua, compile_cvl(lua, content).unwrap());
}

/// Run a lua string and *return* the result
pub fn eval<R: for<'lua> FromLuaMulti<'lua>>(lua: &Lua, content: String) -> LuaResult<R> {
    return lua.context(move |lua_ctx| {
        return lua_ctx.load(content.as_str()).eval();
    });
}

pub fn eval_cvl<R: for<'lua> FromLuaMulti<'lua>>(lua: &Lua, content: String) -> LuaResult<R> {
    return eval(lua, compile_cvl(lua, content).unwrap());
}
