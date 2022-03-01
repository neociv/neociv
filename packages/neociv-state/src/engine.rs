use crate::civ::{Civ, CivId};
use crate::state::NeocivState;
use crate::errors::*;

/// Any modification to the state produces a result that either contains the successfully updated
/// state *or* fails with a specific StateError.
pub type StateResult = Result<NeocivState, StateError>;

/// Return a state that is deemed succesful. Most importantly increment the revision counter. This
/// should be used *everywhere* that an action is performed that *changes* the state in any way and
/// that change is successfully applied.
macro_rules! return_next_state {
    ($state:expr) => {
        $state.revision += 1;
        return Ok($state);
    }
}

/// Initialise an empty & default state.
pub fn init() -> NeocivState {
    return NeocivState::default();
}

/// Add a Civ to the state.
pub fn add_civ(state: NeocivState, civ: Civ) -> StateResult {
    if state.civs.iter().any(|i| i.id == civ.id) {
        err_dup_civ!(civ.id);
    } else if civ.id.len() == 0 {
        err_invalid_civ!(civ.id);
    } else {
        let mut new_state = state.clone();
        new_state.civs.push(civ);
        return_next_state!(new_state);
    }
}

/// Remove a Civ from the state - including from all ownership roles and owned units
pub fn remove_civ(state: NeocivState, civ_id: CivId) -> StateResult {
    if civ_id.len() == 0 {
        err_invalid_civ!(civ_id);
    } else if !state.civs.iter().any(|i| i.id == civ_id) {
        err_unknown_civ!(civ_id);
    } else {
        let mut new_state = state.clone();
        // TODO: Remove ownership of all cells
        // TODO: Remove ownership of all units
        // Remove from the list of civs
        new_state.civs.retain(|c| c.id != civ_id);
        return_next_state!(new_state);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init() {
        let state: crate::state::NeocivState = crate::engine::init();
        assert_eq!(state.revision, 0);
        assert_eq!(state.turn, 0);
        assert_eq!(state.civs.len(), 0);
        assert_eq!(state.grid.cells.len(), 0);
    }

    #[test]
    fn test_add_civ() {
        let mut state: crate::state::NeocivState = crate::engine::init();
        assert_eq!(state.civs.len(), 0);
        assert_eq!(state.revision, 0);
        state = match crate::engine::add_civ(
            state,
            crate::civ::Civ {
                id: String::from("example"),
                title: String::from("Example"),
                ..Default::default()
            },
        ) {
                Ok(state) => state,
                Err(ex) => panic!("{:?}", ex),
            };
        assert_eq!(state.civs.len(), 1);
        assert_eq!(state.revision, 1);
    }

    #[test]
    fn test_err_dup_civ() {
        let mut state: crate::state::NeocivState = crate::engine::init();
        assert_eq!(state.civs.len(), 0);
        assert_eq!(state.revision, 0);
        state = match crate::engine::add_civ(
            state,
            crate::civ::Civ {
                id: String::from("example"),
                title: String::from("Example"),
                ..Default::default()
            },
        ) {
                Ok(state) => state,
                Err(ex) => panic!("{:?}", ex),
            };
        assert_eq!(state.civs.len(), 1);
        assert_eq!(state.revision, 1);
        // Add a Civ with the same Id to cause an error
        let error = crate::engine::add_civ(
            state,
            crate::civ::Civ {
                id: String::from("example"),
                title: String::from("Example"),
                ..Default::default()
            },
        );
        assert_eq!(error.is_ok(), false);
        assert_eq!(error.unwrap_err().code, crate::errors::StateErrorCode::DuplicateCivId);
    }

    #[test]
    fn test_remove_civ() {
        let mut state: crate::state::NeocivState = crate::engine::init();
        assert_eq!(state.civs.len(), 0);
        assert_eq!(state.revision, 0);
        state = match crate::engine::add_civ(
            state,
            crate::civ::Civ {
                id: String::from("example"),
                title: String::from("Example"),
                ..Default::default()
            },
        ) {
                Ok(state) => state,
                Err(ex) => panic!("{:?}", ex),
            };
        assert_eq!(state.civs.len(), 1);
        assert_eq!(state.revision, 1);
        state = match crate::engine::remove_civ(state, String::from("example")) {
            Ok(state) => state,
            Err(ex) => panic!("{:?}", ex),
        };
        assert_eq!(state.civs.len(), 0);
        assert_eq!(state.revision, 2);
    }
}
