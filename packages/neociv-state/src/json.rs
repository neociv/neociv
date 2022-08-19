use serde_json;

use crate::state::NeocivState;

/// Serialize the state as JSON.
pub fn to_json(state: &NeocivState) -> Result<String, serde_json::Error> {
    return serde_json::to_string(state);
}

/// Parse the JSON representation of state.
pub fn from_json(json: &str) -> Result<NeocivState, serde_json::Error> {
    return serde_json::from_str(json);
}

#[test]
fn test_defaults() {
    assert!(to_json(&crate::engine::init()).is_ok());
    assert!(from_json(to_json(&crate::engine::init()).unwrap().as_str()).is_ok());
}
