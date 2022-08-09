use indoc::formatdoc;
use std::{error::Error, fs::read_to_string};
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::path::Path;
use std::result::Result;

use rlua::{Error as LuaError, FromLuaMulti, Lua, Result as LuaResult};

static FENNEL_FILE: &'static str = include_str!("fennel.lua");

#[derive(Debug)]
pub enum CvlError {
    LuaError(LuaError),
    FileNotFound,
    UnknownFileType,
}

impl Error for CvlError {}

impl Display for CvlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

pub fn compile_cvl(lua: &Lua, content: &str) -> Result<String, LuaError> {
    let block = formatdoc! {r#"
            local fnl_compiler = require("fennel.compiler") 
            return fnl_compiler["compile-string"]([[
                {}
            ]])
        "#, content};

    return eval::<String>(lua, block.as_str());
}

/// Load a file into the lua context, appropriately deciding whether to parse it as cvl (fennel) or lua.
pub fn load_file<'a>(lua: &'a Lua, filepath: &str) -> Result<&'a Lua, CvlError> {
    let path = Path::new(filepath);

    if !path.exists() {
        return Err(CvlError::FileNotFound);
    } else if path.extension().unwrap() == "lua" {
        return match load_lua_str(lua, read_to_string(path.as_os_str()).unwrap().as_str()) {
            Ok(_) => Ok(lua),
            Err(ex) => Err(CvlError::LuaError(ex)),
        };
    } else if path.extension().unwrap() == "fnl" || path.extension().unwrap() == "cvl" {
        return match load_cvl_str(lua, read_to_string(path.as_os_str()).unwrap().as_str()) {
            Ok(_) => Ok(lua),
            Err(ex) => Err(CvlError::LuaError(ex))
        }
    } else {
        return Err(CvlError::UnknownFileType);
    }
}

/// Load a string containing lua into the context only caring if successfully parsed or not.
pub fn load_lua_str<'a>(lua: &'a Lua, content: &str) -> Result<&'a Lua, LuaError> {
    lua.context(move |lua_ctx| {
        return lua_ctx.load(content).exec().unwrap();
    });
    return Ok(lua);
}

/// Load a string containing cvl (fennel) into the context by pre-parsing it.
pub fn load_cvl_str<'a>(lua: &'a Lua, content: &str) -> Result<&'a Lua, LuaError> {
    return load_lua_str(lua, compile_cvl(lua, content).unwrap().as_str());
}

/// Run a lua string and *return* the result.
pub fn eval<R: for<'lua> FromLuaMulti<'lua>>(lua: &Lua, content: &str) -> LuaResult<R> {
    return lua.context(move |lua_ctx| {
        return lua_ctx.load(content).eval();
    });
}

/// Run a cvl (fennel) string and *return* the result.
pub fn eval_cvl<R: for<'lua> FromLuaMulti<'lua>>(lua: &Lua, content: &str) -> LuaResult<R> {
    return eval::<R>(lua, compile_cvl(lua, content).unwrap().as_str());
}
