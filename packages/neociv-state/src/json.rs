use crate::engine::init;
use crate::state::NeocivState;

/// Parse the JSON representation of state.
pub fn from_json() -> NeocivState {
    return init();
}

/// Serialize the state as JSON.
pub fn to_json() -> String {
    return stringify!("{}").to_string();
}
