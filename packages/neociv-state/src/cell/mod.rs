use rlua::{Error as LuaError, FromLua, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};

use crate::{state_enum_default, civ::CivKey, mask::CellMasks, utils::opt_str_to_lua};

use self::improvement::Improvement;

pub mod improvement;

state_enum_default! {
    pub enum Terrain {
        DeepWater,
        Water,
        #[default]
        Ground,
    }
}

/// Representation of a single Cell in the Grid.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Cell {
    /// Horizontal (East) position of the cell.
    pub x: u8,
    /// Vertical (North) position of the cell.
    pub y: u8,
    /// Civ that owns this cell, optional.
    pub owner: Option<CivKey>,
    /// Terrain
    pub terrain: Terrain,
    /// Masks
    pub masks: CellMasks,
    /// Improvement
    pub improvement: Option<Improvement>,
}

impl<'lua> ToLua<'lua> for Cell {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let cell_tbl = ctx.create_table()?;
        cell_tbl.set("x", self.x)?;
        cell_tbl.set("y", self.y)?;
        cell_tbl.set("owner", opt_str_to_lua(self.owner, &ctx)?)?;
        cell_tbl.set("masks", self.masks)?;
        cell_tbl.set("terrain", self.terrain)?;
        Ok(LuaValue::Table(cell_tbl))
    }
}

impl<'lua> FromLua<'lua> for Cell {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Cell {
                x: tbl.get("x")?,
                y: tbl.get("y")?,
                owner: tbl.get("owner")?,
                terrain: tbl.get("terrain")?,
                masks: tbl.get("masks")?,
                improvement: tbl.get("improvement")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "Cell",
                message: None,
            }),
        }
    }
}
