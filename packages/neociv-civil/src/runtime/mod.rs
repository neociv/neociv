use neociv_state::state::NeocivState;
use rlua::{
    Error as LuaError, FromLuaMulti, Function as LuaFunction, Lua, Result as LuaResult,
    String as LuaString, Table as LuaTable, Value as LuaValue,
};
use std::path::Path;

use self::engine::engine_do;

pub mod engine;
pub mod errors;
pub mod repl;

static FENNEL_FILE: &'static str = include_str!("./api/fennel.lua");
static SEARCHERS_FILE: &'static str = include_str!("./api/searchers.lua");
static MACROS_FILE: &'static str = include_str!("./api/macros.fnl");
static CVL_FILE: &'static str = include_str!("./api/cvl.lua");

pub struct NeocivRuntime {
    lua: Lua,
    state: NeocivState,
}

impl Default for NeocivRuntime {
    fn default() -> Self {
        unsafe {
            let runtime = NeocivRuntime {
                lua: Lua::new_with_debug(),
                state: NeocivState::default(),
            };
            let _result = runtime.lua.context(move |ctx| {
                ctx.load(FENNEL_FILE).exec()?;
                ctx.load(SEARCHERS_FILE).exec()?;
                ctx.load(CVL_FILE).exec()?;
                let require: LuaFunction = ctx.globals().get("require")?;
                let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
                let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;

                // Include ".cvl" files when searching for modules and macros
                let fennel_path: String = fennel_module.get("path")?;
                let fennel_macro_path: String = fennel_module.get("macro-path")?;
                let path_mod = "./?.cvl;./?/init.cvl;";
                let path_result: String = format!("{}{}", path_mod, fennel_path);
                let macro_path_result: String = format!("{}{}", path_mod, fennel_macro_path);
                fennel_module.set("path", path_result)?;
                fennel_module.set("macro-path", macro_path_result);

                // Perform engine operations
                let do_fn: LuaFunction =
                    ctx.create_function(move |_, (state, action, args): (NeocivState, String, LuaValue)| {
                        let result = engine_do(state, action.as_str(), args)?;
                        Ok(())
                    })?;
                let cvl_tbl: LuaTable = ctx.globals().get("cvl")?;
                return cvl_tbl.set("_engine_do", do_fn);
            });
            return runtime;
        }
    }
}

impl NeocivRuntime {
    /// Create a runtime from a provided state object
    pub fn from(state: NeocivState) -> Self {
        NeocivRuntime {
            state,
            ..NeocivRuntime::default()
        }
    }

    /// Create a runtime from a JSON state file
    pub fn from_file(file: &str) -> Result<Self, ()> {
        return Ok(NeocivRuntime::default());
    }

    pub fn dofile<T: for<'lua> FromLuaMulti<'lua>>(&self, file_str: &str) -> LuaResult<T> {
        let _path_obj = Path::new(file_str);
        if file_str.ends_with(".lua") {
            return self.dofile_lua(file_str);
        } else if file_str.ends_with(".fnl") || file_str.ends_with(".cvl") {
            return self.dofile_fnl(file_str);
        } else {
            return Err(LuaError::RuntimeError(format!(
                "[neociv] Invalid file '{}'",
                file_str
            )));
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
            let lua_state = state.clone();
            return inject_fn.call::<_, ()>(lua_state);
        })?;

        return Ok(self);
    }
}

#[test]
fn test_state_in_lua() {
    let cvl = NeocivRuntime::default();
    let mut state = NeocivState::default();
    let inject_result = cvl.inject_state(&state);

    assert!(inject_result.is_ok());

    let type_result = cvl.eval_lua::<bool>("assert(type(cvl.state) == 'table')");

    assert!(type_result.is_ok());
    assert!(type_result.unwrap());

    let rev_result = cvl.eval_lua::<bool>("assert(cvl.get('revision') == 0)");

    assert!(rev_result.is_ok());
    assert!(rev_result.unwrap());

    let turn_result = cvl.eval_lua::<bool>("assert(cvl.get('turn') == 0)");

    assert!(turn_result.is_ok());
    assert!(turn_result.unwrap());

    let active_civ_nil_result = cvl.eval_lua::<bool>("assert(cvl.get('active_civ_key') == nil)");
    assert!(active_civ_nil_result.is_ok());
    assert!(active_civ_nil_result.unwrap());

    state.active_civ_key = Some(String::from("example"));
    assert!(cvl.inject_state(&state).is_ok());

    let active_civ_result =
        cvl.eval_lua::<bool>("assert(type(cvl.get('active_civ_key')) == 'string')");
    assert!(active_civ_result.is_ok());
    assert!(active_civ_result.unwrap());
}
