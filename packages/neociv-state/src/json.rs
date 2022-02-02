use crate::state::NeocivState;
use crate::engine::init;

/// Parse the JSON representation of state.
pub fn from_json() -> NeocivState {
    return init();
}

/// Serialize the state as JSON.
pub fn to_json() -> String {
    return stringify!("{}").to_string();
}

