use bevy_ecs::prelude::Component;
use rlua::{Nil as LuaNil, ToLua, Value as LuaValue, Error as LuaError, FromLua};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::camera::Camera;
use crate::civ::{Civ, CivKey};
use crate::grid::Grid;
use crate::scales::Scales;

/// Game state structure
#[derive(Clone, Default, Debug, Serialize, Deserialize, Component)]
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
    /// The scales used to control the display of particular values
    pub scales: Scales,
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
        state_tbl.set("scales", self.scales)?;
        Ok(LuaValue::Table(state_tbl))
    }
}

impl<'lua> FromLua<'lua> for NeocivState {
    fn from_lua(lua_value: LuaValue<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(NeocivState { 
                revision: tbl.get("revision")?,
                turn: tbl.get("turn")?,
                active_civ_key: tbl.get("active_civ_key")?,
                civ_order: tbl.get("civ_order")?,
                civs: tbl.get("civs")?,
                grid: tbl.get("grid")?,
                camera: tbl.get("camera")?,
                scales: tbl.get("scales")?,
            }),
            _ => Err(LuaError::FromLuaConversionError { from: lua_value.type_name(), to: "NeocivState", message: None })
        }
    }
}
