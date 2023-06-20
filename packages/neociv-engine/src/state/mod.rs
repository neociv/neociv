use neociv_state::state::NeocivState;
use neociv_state::errors::StateError;

pub fn update_state(path: Option<&str>, current: &NeocivState) -> Result<NeocivState, StateError> {
    return Ok(current.clone());
}
