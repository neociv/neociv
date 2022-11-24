use bevy_math::Vec3;
use neociv_state::civ::CivKey;
use neociv_state::engine::mod_camera_position;
use neociv_state::state::NeocivState;
use rlua::{Error as LuaError, Value as LuaValue};

use super::errors::NeocivRuntimeError;

macro_rules! table_safe_get {
    ($tbl: expr, $key: expr, $default: expr) => {
        match $tbl.get($key) {
            Ok(v) => v,
            Err(_) => $default,
        }
    };
}

fn arg_vec3(v: LuaValue) -> Result<Vec3, NeocivRuntimeError> {
    match v {
        LuaValue::Table(tbl) => Ok(Vec3::new(
            table_safe_get!(tbl, "x", 0.0),
            table_safe_get!(tbl, "y", 0.0),
            table_safe_get!(tbl, "z", 0.0),
        )),
        _ => Err(NeocivRuntimeError::LuaError(
            LuaError::FromLuaConversionError {
                from: v.type_name(),
                to: "Vec3",
                message: None,
            },
        )),
    }
}

fn arg_civkey(v: LuaValue) -> Result<CivKey, NeocivRuntimeError> {
    match v {
        LuaValue::String(str) => Ok(String::from(str.to_str()?)),
        _ => Err(NeocivRuntimeError::LuaError(
            LuaError::FromLuaConversionError {
                from: v.type_name(),
                to: "CivKey",
                message: None,
            },
        )),
    }
}

pub fn engine_do(
    state: NeocivState,
    action: &str,
    args: LuaValue,
) -> Result<NeocivState, NeocivRuntimeError> {
    if action == "mod.camera.position" {
        match mod_camera_position(state, arg_vec3(args)?) {
            Ok(s) => Ok(s),
            Err(ex) => Err(NeocivRuntimeError::from(ex)),
        }
    } else {
        panic!("Unknown action {}", action);
    }
}
