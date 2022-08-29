use serde::{Deserialize, Serialize};
use rlua::{ToLua,FromLua, Value as LuaValue, Error as LuaError};
use bevy_ecs::component::Component;

/// Cell mask's define an 8-bit value with a bit for each edge condition.
/// 
/// 0010 0000 (top-left)      / \    0000 0001 (top-right)
///                          /   \ 
/// 0001 0000 (middle-left) |     |  0000 0010 (middle-right)
///                         |     |
/// 0000 1000 (bottom-left)  \   /   0000 0100 (bottom-right)
///                           \ /
pub type CellMask = u8;

#[derive(Clone, Debug, Serialize, Deserialize, Default, Component)]
pub struct CellMasks {
    /// Whether or not the neighbour shares the same owner
    pub owner: CellMask,
    /// Whether or not the neighbour is the same terrain type
    pub terrain: CellMask,
    /// Whether or not the neighbour is water while the cell is ground
    pub coast: CellMask,
    /// Whether or not the neighbour has a road (or equivalent) on it
    pub road: CellMask,
}

impl<'lua> ToLua<'lua> for CellMasks {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let masks_tbl = ctx.create_table()?;
        masks_tbl.set("owner", self.owner)?;
        masks_tbl.set("terrain", self.terrain)?;
        masks_tbl.set("coast", self.coast)?;
        masks_tbl.set("road", self.road)?;
        Ok(LuaValue::Table(masks_tbl))
    }
}

impl<'lua> FromLua<'lua> for CellMasks {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(CellMasks {
                owner: tbl.get("owner")?,
                terrain: tbl.get("terrain")?,
                coast: tbl.get("coast")?,
                road: tbl.get("road")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "CellMasks",
                message: None,
            }),
        }
    }
}
