use bevy_math::Vec3;
use rlua::{
    Context, Error as LuaError, Nil as LuaNil, Result as LuaResult, Table as LuaTable,
    Value as LuaValue,
};

/// Quickly converts an appropriate Lua Value to a Vec3
pub fn vec3_from_lua(value: LuaValue) -> Result<Vec3, LuaError> {
    match value {
        LuaValue::Table(tbl) => Ok(Vec3 {
            x: tbl.get("x")?,
            y: tbl.get("y")?,
            z: tbl.get("z")?,
        }),
        _ => Err(LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "Vec3",
            message: Some(String::from("Failed to generate a Vec3 from Lua Value")),
        }),
    }
}

pub fn vec3_from_table(tbl: LuaTable) -> Result<Vec3, LuaError> {
    return Ok(Vec3 {
        x: tbl.get("x")?,
        y: tbl.get("y")?,
        z: tbl.get("z")?,
    });
}

pub fn opt_str_to_lua<'lua>(opt: Option<String>, ctx: &Context<'lua>) -> LuaResult<LuaValue<'lua>> {
    match opt {
        Some(value) => Ok(LuaValue::String(ctx.create_string(value.as_str())?)),
        None => Ok(LuaNil),
    }
}
