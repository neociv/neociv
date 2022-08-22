use rlua::{Error as LuaError, FromLua, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alignment {
    pub id: String,
    pub min: String,
    pub max: String,
    pub default: f32,
    pub value: f32,
}

impl<'lua> ToLua<'lua> for Alignment {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let align_tbl = ctx.create_table()?;
        align_tbl.set("id", self.id)?;
        align_tbl.set("min", self.min)?;
        align_tbl.set("max", self.max)?;
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
                min: table.get::<_, String>("min")?,
                max: table.get::<_, String>("max")?,
                default: table.get::<_, f32>("default")?,
                value: table.get::<_, f32>("value")?,
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
