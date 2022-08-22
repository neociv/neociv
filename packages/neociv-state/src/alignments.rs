use rlua::{ToLua, Value as LuaValue};
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

pub type Alignments = HashMap<String, Alignment>;
