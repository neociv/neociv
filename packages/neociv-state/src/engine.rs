use crate::civ::{Civ, CivId};
use crate::state::NeocivState;

#[derive(Debug)]
pub enum StateError {
    DuplicateCivId,
    InvalidCivId,
    UnknownCivId,
}

/// Any modification to the state produces a result that either contains the successfully updated
/// state *or* fails with a specific StateError.
pub type StateResult = Result<NeocivState, StateError>;

/// Initialise an empty & default state.
pub fn init() -> NeocivState {
    return NeocivState::default();
}

/// Add a Civ to the state.
pub fn add_civ(state: NeocivState, civ: Civ) -> StateResult {
    if state.civs.iter().any(|i| i.id == civ.id) {
        return Err(StateError::DuplicateCivId);
    } else if civ.id.len() == 0 {
        return Err(StateError::InvalidCivId);
    } else {
        let mut new_state = state.clone();
        new_state.civs.push(civ);
        return Ok(new_state);
    }
}

/// Remove a Civ from the state - including from all ownership roles and owned units
pub fn remove_civ(state: NeocivState, civ_id: CivId) -> StateResult {
    if civ_id.len() == 0 {
        return Err(StateError::InvalidCivId);
    } else if !state.civs.iter().any(|i| i.id == civ_id) {
        return Err(StateError::UnknownCivId);
    } else {
        let mut new_state = state.clone();
        // TODO: Remove ownership of all cells
        // TODO: Remove ownership of all units
        // Remove from the list of civs
        new_state.civs.retain(|c| c.id != civ_id);
        return Ok(new_state);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init() {
        let state: crate::state::NeocivState = crate::engine::init();
        assert_eq!(state.civs.len(), 0);
        assert_eq!(state.grid.cells.len(), 0);
    }

    #[test]
    fn test_add_civ() {
        let mut state: crate::state::NeocivState = crate::engine::init();
        assert_eq!(state.civs.len(), 0);
        state = match crate::engine::add_civ(
            state,
            crate::civ::Civ {
                id: String::from("example"),
                title: String::from("Example"),
            },
        ) {
            Ok(state) => state,
            Err(ex) => panic!("{:?}", ex),
        };
        assert_eq!(state.civs.len(), 1);
    }

    #[test]
    fn test_remove_civ() {
        let mut state: crate::state::NeocivState = crate::engine::init();
        assert_eq!(state.civs.len(), 0);
        state = match crate::engine::add_civ(
            state,
            crate::civ::Civ {
                id: String::from("example"),
                title: String::from("Example"),
            },
        ) {
            Ok(state) => state,
            Err(ex) => panic!("{:?}", ex),
        };
        assert_eq!(state.civs.len(), 1);
        state = match crate::engine::remove_civ(state, String::from("example")) {
            Ok(state) => state,
            Err(ex) => panic!("{:?}", ex),
        };
        assert_eq!(state.civs.len(), 0);
    }
}
