use bevy_math::Vec3;
use neociv_state::alignments::Alignment;
use neociv_state::civ::CivKey;
use neociv_state::engine::{mod_camera_position, StateResult};
use neociv_state::errors::{StateError, StateErrorCode};
use neociv_state::state::NeocivState;
use neociv_state::{err_unknown_engine_action, state_error, state_panic};
use rlua::{Error as LuaError, FromLua, Table as LuaTable, Value as LuaValue};
use std::error::Error;

use super::errors::NeocivRuntimeError;

fn arg_vec3(v: LuaValue) -> Result<Vec3, NeocivRuntimeError> {
    match v {
        LuaValue::Table(tbl) => Ok(Vec3::new(tbl.get("x")?, tbl.get("y")?, tbl.get("z")?)),
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
    if action == "mod_camera_position" {
        match mod_camera_position(state, arg_vec3(args)?) {
            Ok(s) => Ok(s),
            Err(ex) => Err(NeocivRuntimeError::from(ex)),
        }
    } else {
        panic!("Unknown action {}", action);
    }
}
