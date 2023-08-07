use neociv_state::{alignments::Alignment, state::NeocivState};
use rlua::{Error as LuaError, FromLua, Lua, Table as LuaTable, ToLua};

#[test]
fn from_lua_defaults() {
    let lua = Lua::new();

    let b: Result<(), LuaError> = lua.context(|ctx| {
        // Create a table *from* the default state
        let state_tbl: LuaTable = LuaTable::from_lua(NeocivState::default().to_lua(ctx)?, ctx)?;

        // Create a NeocivState instance by converting the default to lua and back again
        let from_lua_state = NeocivState::from_lua(NeocivState::default().to_lua(ctx)?, ctx)?;

        assert_eq!(
            from_lua_state.revision,
            state_tbl.get::<_, u64>("revision")?
        );
        assert_eq!(from_lua_state.turn, state_tbl.get::<_, u32>("turn")?);

        Ok(())
    });

    assert!(b.is_ok());
}
