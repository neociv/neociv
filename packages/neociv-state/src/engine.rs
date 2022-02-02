use crate::state::NeocivState;
use crate::civ::Civ;

pub enum StateError {
    DuplicateCivId,
    InvalidCivId,
}

pub type StateResult = Result<NeocivState, StateError>;

/// Initialise an empty & default state.
pub fn init() -> NeocivState {
    return NeocivState::default();
}

/// Add a Civ to the state.
pub fn add_civ(mut state: NeocivState, civ: Civ ) -> StateResult {
    if state.civs.iter().any(|i| i.id == civ.id) {
        return Err(StateError::DuplicateCivId);
    } else if civ.id.len() == 0 {
        return Err(StateError::InvalidCivId);
    } else {
        state.civs.push(civ);
        return Ok(state);
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
        state = crate::engine::add_civ(state, crate::civ::Civ { id: String::from("example"), title: String::from("Example") });
        assert_eq!(state.civs.len(), 1);
    }
}
