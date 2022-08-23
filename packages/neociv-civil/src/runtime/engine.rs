use bevy_math::Vec3;
use neociv_state::alignments::Alignment;
use neociv_state::engine::{mod_camera_position, StateResult};
use neociv_state::errors::{StateError, StateErrorCode};
use neociv_state::state::NeocivState;
use neociv_state::{err_unknown_engine_action, state_error, state_panic};
use rlua::{Error as LuaError, FromLua, Table as LuaTable, Value as LuaValue};

pub fn engine_do(state: NeocivState, action: &str, args: LuaValue) -> Result<NeocivState, LuaError> {
    if action == "mod_camera_position" {
        let pos = match args {
            LuaValue::Table(tbl) => Ok(Vec3::new(tbl.get("x")?, tbl.get("y")?, tbl.get("z")?)),
            _ => Err(LuaError::FromLuaConversionError {
                from: args.type_name(),
                to: "Vec3",
                message: None,
            }),
        }?;
        let result = mod_camera_position(state, pos);
        return Ok(result.unwrap());
    } else {
        panic!("Unknown action {}", action);
    }
}
