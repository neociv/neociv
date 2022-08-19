use crate::runtime::lua::{CvlError, NeocivLuaRuntime};
use std::path::Path;

use neociv_state::state::NeocivState;
use crate::sdk::rlua::state::state_to_lua;

use rlua::{
    Error as LuaError, FromLuaMulti, Function as LuaFunction, Lua as RLua, String as LuaString,
    Table as LuaTable,
};

pub mod state;

static FENNEL_FILE: &'static str = include_str!("../../runtime/api/fennel.lua");
static SEARCHERS_FILE: &'static str = include_str!("../../runtime/api/searchers.lua");
static MACROS_FILE: &'static str = include_str!("../../runtime/api/macros.fnl");
static CVL_FILE: &'static str = include_str!("../../runtime/api/cvl.lua");

impl NeocivLuaRuntime for RLua {
    fn init() -> Result<Self, LuaError>
    where
        Self: Sized,
    {
        unsafe {
            let lua = RLua::new_with_debug();
            lua.context(move |ctx| {
                ctx.load(FENNEL_FILE).exec()?;
                ctx.load(SEARCHERS_FILE).exec()?;
                ctx.load(CVL_FILE).exec()?;
                let require: LuaFunction = ctx.globals().get("require")?;
                let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
                let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;

                // Include ".cvl" files when searching for modules and macros
                let fennel_path: String = fennel_module.get("path")?;
                let fennel_macro_path: String = fennel_module.get("macro-path")?;
                let path_mod = "./?.cvl;";
                let path_result: String = format!("{}{}", path_mod, fennel_path);
                let macro_path_result: String = format!("{}{}", path_mod, fennel_macro_path);
                fennel_module.set("path", path_result)?;
                return fennel_module.set("macro-path", macro_path_result);
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

    fn inject_state<'lua>(&self, state: &NeocivState) -> Result<&Self, LuaError> {
        self.context(move |ctx| {
            let cvl: LuaTable = ctx.globals().get("cvl")?;
            let inject_fn: LuaFunction = cvl.get("inject_state")?;
            let lua_state = state_to_lua(&ctx, state);
            return inject_fn.call::<_, ()>(lua_state);
        })?;
        return Ok(self);
    }
}

#[test]
fn test_rlua_inject_state() {
    let state = NeocivState::default();
    let lua: RLua = NeocivLuaRuntime::init().unwrap();
    assert!(lua.inject_state(&state).is_ok());
}
