use regex::*;

use rlua::{Error as LuaError, FromLua, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};

use crate::cell::improvement::Improvement;

pub type CityKey = String;

lazy_static! {
    pub static ref VALID_CITY_KEY: Regex =
        Regex::new(r"^[a-zA-Z0-9]+\.[a-zA-Z0-9]+(?:\.[a-zA-Z0-9])*\[\d+\]<\d+>$").unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeocivCity {
    pub key: CityKey,
    pub title: String,
    pub capital: bool,
    pub improvement: Improvement,
}

impl<'lua> ToLua<'lua> for NeocivCity {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let city_tbl = ctx.create_table()?;
        city_tbl.set("key", self.key)?;
        city_tbl.set("title", self.title)?;
        city_tbl.set("capital", self.capital)?;
        city_tbl.set("improvement", self.improvement)?;
        Ok(LuaValue::Table(city_tbl))
    }
}

impl<'lua> FromLua<'lua> for NeocivCity {
    fn from_lua(lua_value: LuaValue<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(NeocivCity {
                key: tbl.get("key")?,
                title: tbl.get("title")?,
                capital: tbl.get("capital")?,
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

