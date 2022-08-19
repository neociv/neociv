use neociv_state::state::NeocivState;
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

type NeocivRuntimeError = CvlError;

static FENNEL_FILE: &'static str = include_str!("./api/fennel.lua");
static SEARCHERS_FILE: &'static str = include_str!("./api/searchers.lua");
static MACROS_FILE: &'static str = include_str!("./api/macros.fnl");
static CVL_FILE: &'static str = include_str!("./api/cvl.lua");

pub struct NeocivRuntime {
    lua: Lua,
}

impl Default for NeocivRuntime {
    fn default() -> Self {
        unsafe {
            let lua = Lua::new_with_debug();
            let _result = lua.context(move |ctx| {
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
            });
            return NeocivRuntime { lua };
        }
    }
}

impl NeocivRuntime {
    pub fn dofile<T: for<'lua> FromLuaMulti<'lua>>(&self, file_str: &str) -> LuaResult<T> {
        let _path_obj = Path::new(file_str);
        if file_str.ends_with(".lua") {
            return self.dofile_lua(file_str);
        } else if file_str.ends_with(".fnl") || file_str.ends_with(".cvl") {
            return self.dofile_fnl(file_str);
        } else {
            return Err(LuaError::RuntimeError(format!("[neociv] Invalid file '{}'", file_str)));
        }
    }

    pub fn dofile_lua<T: for<'lua> FromLuaMulti<'lua>>(&self, file_str: &str) -> LuaResult<T> {
        return self.lua.context(move |ctx| {
            let path_str: LuaString = ctx.create_string(file_str)?;
            let dofile: LuaFunction = ctx.globals().get("dofile")?;
            return dofile.call::<_, T>(path_str);
        });
    }

    pub fn eval_lua<T: for<'lua> FromLuaMulti<'lua>>(&self, lua_str: &str) -> LuaResult<T> {
        return self.lua.context(move |ctx| {
            return ctx.load(lua_str).eval();
        });
    }

    pub fn compile_fnl(&self, fnl_str: &str) -> LuaResult<String> {
        return self.lua.context(move |ctx| {
            // Create a lua string containing the provided code
            let code_str: LuaString = ctx.create_string(fnl_str)?;

            // Get the compiler reference
            let require: LuaFunction = ctx.globals().get("require")?;
            let fennel_compiler_ns: LuaTable = require.call::<_, LuaTable>("fennel.compiler")?;
            let fennel_compiler: LuaFunction = fennel_compiler_ns.get("compile-string")?;

            // Get the compiled string
            return fennel_compiler.call::<LuaString, String>(code_str);
        });
    }

    pub fn dofile_fnl<T: for<'lua> FromLuaMulti<'lua>>(&self, file_str: &str) -> LuaResult<T> {
        return self.lua.context(move |ctx| {
            let path_str: LuaString = ctx.create_string(file_str)?;
            let require: LuaFunction = ctx.globals().get("require")?;
            let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
            let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;
            let fennel_dofile: LuaFunction = fennel_module.get("dofile")?;
            return fennel_dofile.call::<_, T>(path_str);
        });
    }

    pub fn eval_fnl<T: for<'lua> FromLuaMulti<'lua>>(&self, fnl_str: &str) -> LuaResult<T> {
        return self.lua.context(move |ctx| {
            let eval_str: LuaString = ctx.create_string(fnl_str)?;
            let require: LuaFunction = ctx.globals().get("require")?;
            let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
            let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;
            let fennel_eval: LuaFunction = fennel_module.get("eval")?;
            return fennel_eval.call::<_, T>(eval_str);
        });
    }

    pub fn inject_state(&self, state: &NeocivState) -> Result<&Self, LuaError> {
        self.lua.context(move |ctx| {
            let cvl: LuaTable = ctx.globals().get("cvl")?;
            let inject_fn: LuaFunction = cvl.get("inject_state")?;
            let lua_state = ctx.create_userdata(state.clone());
            return inject_fn.call::<_, ()>(lua_state);
        })?;

        return Ok(self);
    }
}

#[test]
fn test_state_in_lua() {
    let cvl = NeocivRuntime::default();
    let state = NeocivState::default();
    let inject_result = cvl.inject_state(&state);

    assert!(inject_result.is_ok());

    let type_result = cvl.eval_lua::<bool>("assert(type(cvl.state) == 'userdata')");

    assert!(type_result.is_ok());
    assert!(type_result.unwrap());
}


