use indoc::formatdoc;
use std::fmt::Display;
use std::path::Path;
use std::result::Result;
use std::{error::Error, fs::read_to_string};

use rlua::{
    Error as LuaError, FromLuaMulti, Function as LuaFunction, Lua, Result as LuaResult,
    Table as LuaTable,
};

static FENNEL_FILE: &'static str = include_str!("fennel.lua");
static MACROS_FILE: &'static str = include_str!("./runtime/api/macros.fnl");

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
    unsafe {
        let lua = Lua::new_with_debug();

        let result: LuaResult<()> = lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        let cvl = lua_ctx.create_table()?;

        // Create a key mapping registry
        //cvl.set("_key_maps", lua_ctx.create_table()?)?;

        // Create an event registry
        //cvl.set("_event_registry", lua_ctx.create_table()?)?;

        // "on" will be used to add items to the event registry
        let on = lua_ctx.create_function(|_, _name: String| {
            //println!("Hello, {}!", name);
            return Ok(());
        })?;

        cvl.set("on", on)?;

        globals.set("cvl", cvl)?;

        // Compile in the (slightly modified) fennel API
        lua_ctx.load(FENNEL_FILE).exec()?;

        // Allow the fennel searcher to look for .fnl and .cvl files automatically
        lua_ctx.load(r#"
            table.insert(package.loaders or package.searchers, require("fennel.specials")["make-searcher"]())
        "#).exec()?;

        return Ok(());
    });

        // Load fennel compiler
        //load_lua_str(&lua, FENNEL_FILE)?;

        load_cvl_str(&lua, MACROS_FILE)?;

        return match result {
            Ok(_) => Ok(lua),
            Err(ex) => Err(ex),
        };
    }
}

/// This is such a shonky compiler - there should be some module metadata, proper string construct
/// creation and other niceities in here but for now this'll do.
pub fn compile_cvl(lua: &Lua, content: &str) -> Result<String, LuaError> {
    return lua.context(move |lua_ctx| {
        // Create a lua string
        let cvl_block = lua_ctx.create_string(content).unwrap();

        // Get reference to fennel compiler function
        let require: LuaFunction = lua_ctx.globals().get("require")?;
        let fennel: LuaTable = require.call::<_, LuaTable>("fennel.compiler")?;
        let compiler: LuaFunction = fennel.get("compile-string")?;

        // Get the string from compiler function
        return compiler.call::<_, String>(cvl_block);
    });
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
        return match load_fnl_file(lua, path.to_str().unwrap()) {
            Ok(_) => Ok(lua),
            Err(ex) => Err(CvlError::LuaError(ex)),
        };
    } else {
        return Err(CvlError::UnknownFileType);
    }
}

fn load_fnl_file<'a>(lua: &'a Lua, filepath: &str) -> Result<(), LuaError> {
    return lua.context(move |lua_ctx| {
        let path = lua_ctx.create_string(filepath).unwrap();
        let require: LuaFunction = lua_ctx.globals().get("require")?;
        let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
        let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;
        let fennel_dofile: LuaFunction = fennel_module.get("dofile")?;
        fennel_dofile.call::<_, String>(path)?;
        return Ok(());
    });
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
