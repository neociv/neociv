use rlua::{Error as LuaError, FromLua, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use std::collections::HashMap;
use bevy_ecs::system::Resource;

#[derive(Resource, Clone, Debug, Serialize, Deserialize, Default, SerdeDiff)]
pub struct Alignment {
    pub id: String,
    pub left: String,
    pub right: String,
    pub default: f32,
    pub value: f32,
}

impl<'lua> ToLua<'lua> for Alignment {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let align_tbl = ctx.create_table()?;
        align_tbl.set("id", self.id)?;
        align_tbl.set("left", self.left)?;
        align_tbl.set("right", self.right)?;
        align_tbl.set("default", self.default)?;
        align_tbl.set("value", self.value)?;
        Ok(LuaValue::Table(align_tbl))
    }
}

impl<'lua> FromLua<'lua> for Alignment {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(table) => Ok(Alignment {
                id: table.get::<_, String>("id")?,
                left: table.get::<_, String>("left")?,
                right: table.get::<_, String>("right")?,
                default: 0.0f32,
                value: 0.0f32,
                //default: table.get::<_, f32>("default")?,
                //value: table.get::<_, f32>("value")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "Alignment",
                message: None,
            }),
        }
    }
}

pub type Alignments = HashMap<String, Alignment>;
