use rlua::{
    Error as LuaError, FromLua, FromLuaMulti, Nil as LuaNil, String as LuaString, ToLua, UserData,
    UserDataMethods, Value as LuaValue,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::camera::Camera;
use crate::cell::Grid;
use crate::civ::{Civ, CivKey};

/// Game state structure
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct NeocivState {
    /// The current change revision of the state - all actions that change the state will increment
    /// this value which is useful for quickly checking if the state has changed and needs to be
    /// inspected again.
    pub revision: u64,
    /// The turn currently being played - this is intended to tick over when the first Civ begins
    /// playing. Games that are in turn "0" should be considered in an initialisation state.
    pub turn: u32,
    /// Key of the currently active Civ
    pub active_civ_key: Option<CivKey>,
    /// Order of civs to play through which removes inactive civs and avoids issues with ordering
    /// between serialized formats.
    pub civ_order: Vec<CivKey>,
    /// Directory of all Civs
    pub civs: HashMap<CivKey, Civ>,
    /// The grid of cells that make up the game map.
    pub grid: Grid,
    /// The current camera state
    pub camera: Camera,
}

impl<'lua> ToLua<'lua> for NeocivState {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<LuaValue<'lua>> {
        let state_tbl = ctx.create_table()?;
        state_tbl.set("revision", self.revision)?;
        state_tbl.set("turn", self.turn)?;
        state_tbl.set(
            "active_civ_key",
            match self.active_civ_key {
                Some(civ_key) => LuaValue::String(ctx.create_string(&civ_key)?),
                None => LuaNil,
            },
        )?;
        let civ_order_seq = ctx.create_sequence_from(self.civ_order)?;
        state_tbl.set("civ_order", civ_order_seq)?;
        let civs_tbl = ctx.create_table_from(self.civs)?;
        state_tbl.set("civs", civs_tbl)?;
        state_tbl.set("grid", self.grid)?;
        state_tbl.set("camera", self.camera)?;
        Ok(LuaValue::Table(state_tbl))
    }
}
