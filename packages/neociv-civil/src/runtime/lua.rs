use rlua::{
    Error as LuaError, FromLuaMulti, Function as LuaFunction, Lua, Result as LuaResult,
    String as LuaString, Table as LuaTable,
};
use std::error::Error;
use std::fmt::Display;
use std::path::Path;

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

static FENNEL_FILE: &'static str = include_str!("./api/fennel.lua");
static SEARCHERS_FILE: &'static str = include_str!("./api/searchers.lua");
static CVL_FILE: &'static str = include_str!("./api/cvl.lua");

pub trait NeocivLuaRuntime {
    fn init() -> Result<Self, LuaError>
    where
        Self: Sized;
    fn dofile(&self, filepath: &str) -> Result<&Self, CvlError>
    where
        Self: Sized;

    fn dofile_lua(&self, filepath: &str) -> Result<&Self, LuaError>
    where
        Self: Sized;
    fn eval_lua<R: for<'lua> FromLuaMulti<'lua>>(&self, lua_str: &str) -> Result<R, LuaError>
    where
        Self: Sized;

    fn compile_fnl(&self, fnl_str: &str) -> Result<String, LuaError>;
    fn dofile_fnl(&self, filepath: &str) -> Result<&Self, LuaError>
    where
        Self: Sized;
    fn eval_fnl<R: for<'lua> FromLuaMulti<'lua>>(&self, fnl_str: &str) -> Result<R, LuaError>
    where
        Self: Sized;
}

impl NeocivLuaRuntime for Lua {
    fn init() -> Result<Self, LuaError>
    where
        Self: Sized,
    {
        unsafe {
            let lua = Lua::new_with_debug();
            lua.context(move |ctx| {
                ctx.load(FENNEL_FILE).exec()?;
                ctx.load(SEARCHERS_FILE).exec()?;
                return ctx.load(CVL_FILE).exec();
            })?;
            return Ok(lua);
        }
    }

    fn dofile(&self, filepath: &str) -> Result<&Self, CvlError>
    where
        Self: Sized,
    {
        let path_obj = Path::new(filepath);
        if !path_obj.exists() {
            return Err(CvlError::FileNotFound);
        } else if path_obj.extension().unwrap() == "lua" {
            return match self.dofile_lua(filepath) {
                Ok(_) => Ok(self),
                Err(ex) => Err(CvlError::LuaError(ex)),
            };
        } else if path_obj.extension().unwrap() == "fnl" || path_obj.extension().unwrap() == "cvl" {
            return match self.dofile_fnl(filepath) {
                Ok(_) => Ok(self),
                Err(ex) => Err(CvlError::LuaError(ex)),
            };
        } else {
            return Err(CvlError::UnknownFileType);
        }
    }

    fn dofile_lua(&self, filepath: &str) -> Result<&Self, LuaError>
    where
        Self: Sized,
    {
        self.context(move |ctx| {
            let path_str: LuaString = ctx.create_string(filepath)?;
            let dofile: LuaFunction = ctx.globals().get("dofile")?;
            return dofile.call::<_, ()>(path_str);
        })?;
        return Ok(self);
    }

    fn eval_lua<R: for<'lua> FromLuaMulti<'lua>>(&self, lua_str: &str) -> Result<R, LuaError> {
        return self.context(move |ctx| {
            return ctx.load(lua_str).eval();
        });
    }

    fn compile_fnl(&self, fnl_str: &str) -> Result<String, LuaError> {
        return self.context(move |ctx| {
            // Create a lua string containing the provided code
            let code_str: LuaString = ctx.create_string(fnl_str)?;

            // Get the compiler reference
            let require: LuaFunction = ctx.globals().get("require")?;
            let fennel_compiler_ns: LuaTable = require.call::<_, LuaTable>("fennel.compiler")?;
            let fennel_compiler: LuaFunction = fennel_compiler_ns.get("compile-string")?;

            // Get the compiled string
            return fennel_compiler.call::<_, String>(code_str);
        });
    }

    fn dofile_fnl(&self, filepath: &str) -> Result<&Self, LuaError>
    where
        Self: Sized,
    {
        self.context(move |ctx| {
            let path_str: LuaString = ctx.create_string(filepath)?;
            let require: LuaFunction = ctx.globals().get("require")?;
            let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
            let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;
            let fennel_dofile: LuaFunction = fennel_module.get("dofile")?;
            return fennel_dofile.call::<_, ()>(path_str);
        })?;
        return Ok(self);
    }

    fn eval_fnl<R: for<'lua> FromLuaMulti<'lua>>(&self, fnl_str: &str) -> Result<R, LuaError>
    where
        Self: Sized,
    {
        return self.context(move |ctx| {
            let eval_str: LuaString = ctx.create_string(fnl_str)?;
            let require: LuaFunction = ctx.globals().get("require")?;
            let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
            let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;
            let fennel_eval: LuaFunction = fennel_module.get("eval")?;
            return fennel_eval.call::<LuaString, R>(eval_str);
        });
    }
}

pub fn init_lua() -> Result<impl NeocivLuaRuntime, LuaError> {
    return init_lua_sdk("rlua");
}

pub fn init_lua_sdk(sdk: &str) -> Result<impl NeocivLuaRuntime, LuaError> {
    if sdk.eq("rlua") {
        return Lua::init();
    } else {
        panic!("Unknown lua sdk '{}'", sdk);
    }
}
