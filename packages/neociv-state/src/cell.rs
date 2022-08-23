use rlua::{FromLua, Nil as LuaNil, ToLua, Value as LuaValue, Error as LuaError};
use serde::{Deserialize, Serialize};

use crate::{civ::CivKey, utils::opt_str_to_lua};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Terrain {
    DeepWater,
    Water,
    Ground,
}

/// Representation of a single Cell in the Grid.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    /// Horizontal (East) position of the cell.
    pub x: u8,
    /// Vertical (North) position of the cell.
    pub y: u8,
    /// Civ that owns this cell, optional.
    pub owner: Option<CivKey>,
    /// Terrain
    pub terrain: Option<Terrain>,
}

impl<'lua> ToLua<'lua> for Cell {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let cell_tbl = ctx.create_table()?;
        cell_tbl.set("x", self.x)?;
        cell_tbl.set("y", self.y)?;
        cell_tbl.set("owner", opt_str_to_lua(ctx, self.owner)?)?;
        Ok(LuaValue::Table(cell_tbl))
    }
}

impl<'lua> FromLua<'lua> for Cell {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok({}),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "Cell",
                message: None,
            }),
        }
    }
}
