use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use bevy_ecs::component::Component;
use bevy_ecs::schedule::RunCriteriaLabel;
use bevy_ecs::system::adapter::new;
use bevy_ecs::system::Resource;
use rlua::{
    Context, Error as LuaError, FromLuaMulti, Function as LuaFunction, Lua, Result as LuaResult,
    String as LuaString, Table as LuaTable, Value as LuaValue,
};

use neociv_state::state::NeocivState;

use self::{engine::engine_do, errors::NeocivRuntimeError};
use crate::content::manifest::NeocivManifest;
use crate::desc::NeocivDesc;

pub mod engine;
pub mod errors;
pub mod repl;
pub mod utils;

static FENNEL_FILE: &'static str = include_str!("./api/vendor/fennel.lua");
static INSPECT_FILE: &'static str = include_str!("./api/vendor/inspect.lua");
static LODASH_FILE: &'static str = include_str!("./api/vendor/lodash.lua");
static LUNAJSON_DECODER_FILE: &'static str = include_str!("./api/vendor/lunajson-decoder.lua");
static LUNAJSON_ENCODER_FILE: &'static str = include_str!("./api/vendor/lunajson-encoder.lua");
static LUNAJSON_SAX_FILE: &'static str = include_str!("./api/vendor/lunajson-sax.lua");
static LUNAJSON_INIT_FILE: &'static str = include_str!("./api/vendor/lunajson.lua");
static SEARCHERS_FILE: &'static str = include_str!("./api/searchers.lua");
static MACROS_FILE: &'static str = include_str!("./api/macros.fnl");
static CVL_FILE: &'static str = include_str!("./api/cvl.lua");
static EVENTS_FILE: &'static str = include_str!("./api/events.fnl");

#[derive(Clone, Debug, Component, Resource)]
pub struct NeocivRuntime {
    lua: Arc<Mutex<Lua>>,
    pub state: NeocivState,
}

impl Default for NeocivRuntime {
    fn default() -> Self {
        unsafe {
            let runtime = NeocivRuntime {
                lua: Arc::new(Mutex::new(Lua::new_with_debug())),
                state: NeocivState::default(),
            };
            let _result = runtime.lua.lock().unwrap().context(move |ctx| {
                ctx.load(INSPECT_FILE).exec()?;
                ctx.load(LODASH_FILE).exec()?;
                ctx.load(LUNAJSON_DECODER_FILE).exec()?;
                ctx.load(LUNAJSON_ENCODER_FILE).exec()?;
                ctx.load(LUNAJSON_SAX_FILE).exec()?;
                ctx.load(LUNAJSON_INIT_FILE).exec()?;
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
                fennel_module.set("macro-path", macro_path_result)?;

                ctx.set_named_registry_value("state", NeocivState::default())?;

                // Perform engine operations
                let do_fn: LuaFunction = ctx.create_function(
                    |fn_ctx, (action, args): (String, LuaValue)| match engine_do(
                        fn_ctx.named_registry_value("state")?,
                        action.as_str(),
                        args,
                    ) {
                        Ok(s) => NeocivRuntime::inject_state_into_context(&fn_ctx, &s),
                        _ => panic!("Oh no!"),
                    },
                )?;

                let load_content_file: LuaFunction =
                    ctx.create_function(|fn_ctx, filename: String| {
                        // Confirm the file exists
                        let cf_path = Path::new(filename.as_str());

                        if cf_path.exists() && !cf_path.is_dir() {
                            // Get path to parent directory of content file and then ensure it is absolute
                            let base_path = cf_path
                                .parent()
                                .expect("Unable to get parent path to content file")
                                .canonicalize()
                                .expect("Unable to get absolute path to content file directory")
                                .as_path()
                                .to_owned();

                            // Get the path to the manifest
                            let manifest = match NeocivManifest::from_child_path_str(
                                base_path.to_str().unwrap(),
                            ) {
                                Some(m) => m,
                                None => panic!("Unable find manifest"),
                            };

                            // Read the file into JSON
                            let json_src = fs::read_to_string(cf_path).unwrap();

                            // Get reference to lunajson
                            let fn_require: LuaFunction = fn_ctx.globals().get("require")?;
                            let lunajson_decode: LuaFunction =
                                fn_require.call::<_, LuaTable>("lunajson")?.get("decode")?;

                            // Turn into a lua table
                            let mut content_tbl: LuaTable =
                                lunajson_decode.call::<_, LuaTable>(json_src)?;

                            // Turn into an array if not already one
                            if content_tbl.len()? == 0 {
                                let arr_table = fn_ctx.create_table()?;
                                arr_table.set(0, content_tbl)?;
                                content_tbl = arr_table;
                            }

                            // Iterate over each entry in the table by index - going by "pairs" doesn't work
                            // so well as it moves the table making it cumbersome to return
                            for i in 1..content_tbl.len()? + 1 {
                                let content: LuaTable = content_tbl.get(i)?;

                                // Replace "@" in the ID with the manfiest ID
                                let content_id = content.get::<_, LuaString>("id")?;
                                if content_id.to_str().unwrap().starts_with("@") {
                                    let new_id = content_id
                                        .to_str()
                                        .unwrap()
                                        .replace("@", manifest.id.as_str());
                                    content.set("id", new_id)?;
                                }

                                let resources: LuaTable = content.get("resources")?;
                                let new_resources = fn_ctx.create_table()?;

                                // Iterate over all the entries in the resource table and create absolute paths
                                // to each one, except for the materials which are organised by namespace
                                for rpair in resources.pairs::<LuaString, LuaValue>() {
                                    let pair = rpair?;
                                    let key = pair.0;
                                    if key.eq(&String::from("materials")) {
                                        let materials: LuaTable =
                                            fn_ctx.unpack::<LuaTable>(pair.1)?;
                                        for n in 1..materials.len()? + 1 {
                                            let material_str = materials.get::<_, LuaString>(n)?;
                                            let m_id = material_str
                                                .to_str()
                                                .unwrap()
                                                .replace("@", manifest.id.as_str());
                                            materials.set(n, m_id)?;
                                        }
                                        new_resources.set::<_, LuaTable>(key, materials)?;
                                    } else {
                                        // Coerce the Lua String containing the resource path into a Rust String
                                        let resource_str = fn_ctx
                                            .coerce_string(pair.1)
                                            .unwrap()
                                            .expect("Unable to coerce resource value");

                                        // Create a new Path buffer from the provided resource String
                                        let resource_path =
                                            Path::new(resource_str.to_str().unwrap());

                                        // Join that to the absolute path of the directory
                                        // containing the content file
                                        let new_path = fn_ctx.create_string(
                                            base_path.join(resource_path).to_str().unwrap(),
                                        )?;
                                        // Set this value in the new resources table
                                        new_resources.set::<_, LuaString>(key, new_path)?;
                                    }
                                }

                                content.set::<_, LuaTable>("resources", new_resources)?;
                            }

                            return LuaResult::Ok(content_tbl);
                        } else {
                            panic!("Failed to find content file '{}'", filename);
                        }
                    })?;

                let cvl_tbl: LuaTable = ctx.globals().get("cvl")?;
                let native_table: LuaTable = cvl_tbl.get("native")?;

                native_table.set("engine_do", do_fn)?;
                native_table.set("load_content_file", load_content_file)?;

                return LuaResult::Ok(());
            });

            // Setup basic events
            match runtime.eval_fnl::<()>(EVENTS_FILE) {
                Ok(_) => (),
                Err(_) => panic!("events failed to parse"),
            }

            return runtime;
        }
    }
}

impl NeocivRuntime {
    /// Create a runtime from a provided state object
    pub fn from(state: NeocivState) -> Result<Self, NeocivRuntimeError> {
        let mut base = NeocivRuntime::default();
        return match base.inject_state(&state.clone()) {
            Ok(_) => Ok(base),
            Err(ex) => Err(NeocivRuntimeError::LuaError(ex)),
        };
    }

    /// Create a runtime from a JSON state file
    pub fn from_file(file: &str) -> Result<Self, NeocivRuntimeError> {
        let path_obj = Path::new(file);
        if !path_obj.exists() {
            return Err(NeocivRuntimeError::FileNotFound);
        } else {
            let json_str = fs::read_to_string(file).unwrap();
            return Ok(NeocivRuntime::default());
        }
    }

    /// Pulls the state out from the Lua context
    pub fn get_state(&self) -> LuaResult<NeocivState> {
        return self.lua.lock().unwrap().context(move |ctx| {
            return ctx.named_registry_value("state");
        });
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
        return self.lua.lock().unwrap().context(move |ctx| {
            let path_str: LuaString = ctx.create_string(file_str)?;
            let dofile: LuaFunction = ctx.globals().get("dofile")?;
            return dofile.call::<_, T>(path_str);
        });
    }

    pub fn eval_lua<T: for<'lua> FromLuaMulti<'lua>>(&self, lua_str: &str) -> LuaResult<T> {
        return self.lua.lock().unwrap().context(move |ctx| {
            return ctx.load(lua_str).eval();
        });
    }

    pub fn compile_fnl(&self, fnl_str: &str) -> LuaResult<String> {
        return self.lua.lock().unwrap().context(move |ctx| {
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
        return self.lua.lock().unwrap().context(move |ctx| {
            let path_str: LuaString = ctx.create_string(file_str)?;
            let require: LuaFunction = ctx.globals().get("require")?;
            let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
            let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;
            let fennel_dofile: LuaFunction = fennel_module.get("dofile")?;
            return fennel_dofile.call::<_, T>(path_str);
        });
    }

    pub fn eval_fnl<T: for<'lua> FromLuaMulti<'lua>>(&self, fnl_str: &str) -> LuaResult<T> {
        return self.lua.lock().unwrap().context(move |ctx| {
            let eval_str: LuaString = ctx.create_string(fnl_str)?;
            let require: LuaFunction = ctx.globals().get("require")?;
            let fennel_utils: LuaTable = require.call::<_, LuaTable>("fennel.utils")?;
            let fennel_module: LuaTable = fennel_utils.get("fennel-module")?;
            let fennel_eval: LuaFunction = fennel_module.get("eval")?;
            return fennel_eval.call::<_, T>(eval_str);
        });
    }

    pub fn inject_state_into_context(ctx: &Context, state: &NeocivState) -> LuaResult<()> {
        ctx.set_named_registry_value("state", state.clone())?;
        let cvl: LuaTable = ctx.globals().get("cvl")?;
        let inject_fn: LuaFunction = cvl.get("inject_state")?;
        return inject_fn.call::<_, ()>(state.clone());
    }

    pub fn inject_state(&mut self, state: &NeocivState) -> Result<&Self, LuaError> {
        self.lua
            .lock()
            .unwrap()
            .context(move |ctx| NeocivRuntime::inject_state_into_context(&ctx, state))?;

        self.state = state.clone();
        return Ok(self);
    }

    pub fn update(&mut self) -> Result<&Self, LuaError> {
        let lua_state = self.get_state();
        return match lua_state {
            Ok(s) => {
                self.state = s;
                return Ok(self);
            }
            Err(ex) => Err(ex),
        };
    }

    /**
     * Inspects the content queue and generates a load order for it such that all dependencies
     * are satisfied.
     */
    pub fn get_load_order(&self) -> LuaResult<Vec<String>> {
        return self.lua.lock().unwrap().context(move |ctx| {
            let cvl = ctx.globals().get::<_, LuaTable>("cvl")?;
            let cvl_content_queue: LuaTable = cvl.get("content_queue")?;
            let mut queue: Vec<String> = vec![];
            let mut content_queue: Vec<String> = vec![];
            // There's not much complexity here - just pull out the materials and put them first
            for pair in cvl_content_queue.clone().pairs::<LuaString, LuaTable>() {
                let (key, entry) = match pair {
                    Ok(p) => p,
                    Err(_) => panic!("Failed to get content pair for ordering"),
                };

                let kind: String = match ctx.coerce_string(entry.get("kind")?)? {
                    Some(s) => String::from(s.to_str().unwrap()),
                    None => panic!("Unable to get kind of content"),
                };

                if kind.contains("material") {
                    queue.push(String::from(key.to_str().unwrap()));
                } else {
                    content_queue.push(String::from(key.to_str().unwrap()));
                }
            }

            // Append the content queue to the end, all materials are now guaranteed to load first
            // content_queue has been moved and is no longer usable after this
            queue.extend(content_queue);

            return Ok(queue);
        });
    }

    pub fn load_content(&self, loader: fn(LuaTable) -> LuaResult<()>) -> LuaResult<()> {
        let order = self.get_load_order().unwrap();

        return self.lua.lock().unwrap().context(move |ctx| {
            let cvl = ctx.globals().get::<_, LuaTable>("cvl")?;
            let content_queue = cvl.get::<_, LuaTable>("content_queue")?;
            for id in order {
                match content_queue.get::<_, LuaTable>(id.as_str()) {
                    Ok(c) => loader(c)?,
                    Err(_) => panic!("Unable to load content with id {:?}", id.as_str()),
                }
            }
            Ok(())
        });
    }
}

#[test]
fn test_state_in_lua() {
    let mut cvl = NeocivRuntime::default();
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
    // Must be done - otherwise the injection won't take as the revision hasn't changed
    // TODO: Write this with engine actions
    state.revision += 1;
    assert!(cvl.inject_state(&state).is_ok());
    assert!(cvl.update().is_ok());

    let active_civ_result =
        cvl.eval_lua::<bool>("assert(type(cvl.get('active_civ_key')) == 'string')");
    assert!(active_civ_result.is_ok());
    assert!(active_civ_result.unwrap());
}
