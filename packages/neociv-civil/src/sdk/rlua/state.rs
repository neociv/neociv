use rlua::{AnyUserData, Context as LuaContext, Error as LuaError};

use neociv_state::state::NeocivState;

use crate::runtime::lua::NeocivLuaRuntime;

pub fn state_to_lua<'lua>(
    ctx: &LuaContext<'lua>,
    state: &NeocivState,
) -> Result<AnyUserData<'lua>, LuaError> {
    return ctx.create_userdata(state.clone());
}

#[test]
fn test_state_in_lua() {
    let lua: rlua::Lua = crate::runtime::lua::NeocivLuaRuntime::init().unwrap();
    let state = NeocivState::default();
    let inject_result = lua.inject_state(&state);

    assert!(inject_result.is_ok());

    let type_result = lua.context(move |ctx| {
        return ctx.load("assert(type(cvl.state) == 'userdata')").exec();
    });

    assert!(type_result.is_ok());
}
