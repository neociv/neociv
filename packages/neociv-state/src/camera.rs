use bevy_math::Vec3;
use rlua::{Error as LuaError, FromLua, Table as LuaTable, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};

use crate::utils::vec3_from_table;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3,
}

impl<'lua> ToLua<'lua> for Camera {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<LuaValue<'lua>> {
        let camera_tbl = ctx.create_table()?;
        let position_tbl = ctx.create_table()?;
        position_tbl.set("x", self.position.x)?;
        position_tbl.set("y", self.position.y)?;
        position_tbl.set("z", self.position.z)?;
        camera_tbl.set("position", position_tbl)?;
        Ok(LuaValue::Table(camera_tbl))
    }
}

impl<'lua> FromLua<'lua> for Camera {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(table) => Ok(Camera {
                position: vec3_from_table(table.get("position")?)?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "Camera",
                message: None,
            }),
        }
    }
}
