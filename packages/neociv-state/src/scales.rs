use rlua::{Error as LuaError, FromLua, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

#[derive(Clone, Debug, Default, Serialize, Deserialize, SerdeDiff)]
pub enum PopulationScale {
    #[default]
    Unit = 1,
    Hundred = 100,
    Thousand = 1_000,
    Million = 1_000_000,
    Billion = 1_000_000_000,
}

impl Into<i64> for PopulationScale {
    fn into(self) -> i64 {
        return self as i64;
    }
}

impl<'lua> ToLua<'lua> for PopulationScale {
    fn to_lua(self, _ctx: rlua::Context<'lua>) -> rlua::Result<LuaValue<'lua>> {
        Ok(LuaValue::Integer(self.into()))
    }
}

impl<'lua> FromLua<'lua> for PopulationScale {
    fn from_lua(lua_value: LuaValue<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Integer(100) => Ok(PopulationScale::Hundred),
            LuaValue::Integer(1_000) => Ok(PopulationScale::Thousand),
            LuaValue::Integer(1_000_000) => Ok(PopulationScale::Million),
            LuaValue::Integer(1_000_000_000) => Ok(PopulationScale::Billion),
            _ => Ok(PopulationScale::Unit),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, SerdeDiff)]
pub enum CurrencyScale {
    #[default]
    Unit = 1,
    Hundred = 100,
    Thousand = 1_000,
    Million = 1_000_000,
    Billion = 1_000_000_000,
    Trillion = 1_000_000_000_000,
}

impl Into<i64> for CurrencyScale {
    fn into(self) -> i64 {
        return self as i64;
    }
}

impl<'lua> ToLua<'lua> for CurrencyScale {
    fn to_lua(self, _lua: rlua::Context<'lua>) -> rlua::Result<LuaValue<'lua>> {
        Ok(LuaValue::Integer(self.into()))
    }
}

impl<'lua> FromLua<'lua> for CurrencyScale {
    fn from_lua(lua_value: LuaValue<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Integer(100) => Ok(CurrencyScale::Hundred),
            LuaValue::Integer(1_000) => Ok(CurrencyScale::Thousand),
            LuaValue::Integer(1_000_000) => Ok(CurrencyScale::Million),
            LuaValue::Integer(1_000_000_000) => Ok(CurrencyScale::Billion),
            LuaValue::Integer(1_000_000_000_000) => Ok(CurrencyScale::Trillion),
            _ => Ok(CurrencyScale::Unit),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff)]
pub struct Scales {
    pub population: PopulationScale,
    pub currency: CurrencyScale,
}

impl<'lua> ToLua<'lua> for Scales {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<LuaValue<'lua>> {
        let scale_tbl = ctx.create_table()?;
        scale_tbl.set("population", self.population)?;
        scale_tbl.set("currency", self.currency)?;
        Ok(LuaValue::Table(scale_tbl))
    }
}

impl<'lua> FromLua<'lua> for Scales {
    fn from_lua(lua_value: LuaValue<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Scales {
                population: tbl.get("population")?,
                currency: tbl.get("currency")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "Scales",
                message: None,
            }),
        }
    }
}
