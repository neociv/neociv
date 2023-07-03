use std::fs;
use std::path::Path;

use rlua::{
    Context as LuaContext, Error as LuaError, Function as LuaFunction, Result as LuaResult,
    String as LuaString, Table as LuaTable, Value as LuaValue,
};

/*
pub fn engine_do<'lua>(ctx: LuaContext<'lua>) -> LuaResult<LuaFunction<'lua>> {
    ctx.create_function(|fn_ctx, (action, args): (String, LuaValue)| {
        match engine_do(fn_ctx.named_registry_value("state")?, action.as_str(), args) {
            Ok(s) => NeocivRuntime::inject_state_into_context(&fn_ctx, &s),
            _ => panic!("Oh no!"),
        }
    })
}
*/
