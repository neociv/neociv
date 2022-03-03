use crate::civ::{Civ, CivId};
use crate::errors::*;
use crate::state::NeocivState;
use crate::cell::Cell;

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
    };
}

/// Return an Err Result with a StateError.
macro_rules! state_panic {
    ($err:expr) => {
        return Err($err);
    };
}

/// Validate the CivId
macro_rules! invalid_civ_id {
    ($id:expr) => {
        $id.len() == 0
    };
}

/// Validate the CivId exists
macro_rules! civ_id_exists {
    ($state:expr, $id:expr) => {
        $state.civs.iter().any(|i| i.id == $id)
    }
}

/// Initialise an empty & default state.
pub fn init() -> NeocivState {
    return NeocivState::default();
}

/// Add a Civ to the state.
pub fn add_civ(state: NeocivState, civ: Civ) -> StateResult {
    if invalid_civ_id!(civ.id) {
        state_panic!(err_invalid_civ!(civ.id));
    } else if civ_id_exists!(state, civ.id) {
        state_panic!(err_dup_civ!(civ.id));
    } else {
        let mut new_state = state.clone();
        new_state.civs.push(civ);
        return_next_state!(new_state);
    }
}

/// Remove a Civ from the state - including from all ownership roles and owned units
pub fn remove_civ(state: NeocivState, civ_id: CivId) -> StateResult {
    if invalid_civ_id!(civ_id) {
        state_panic!(err_invalid_civ!(civ_id));
    } else if !civ_id_exists!(state, civ_id) {
        state_panic!(err_unknown_civ!(civ_id));
    } else {
        let mut new_state = state.clone();
        // TODO: Remove ownership of all cells
        // TODO: Remove ownership of all units
        // Remove from the list of civs
        new_state.civs.retain(|c| c.id != civ_id);
        return_next_state!(new_state);
    }
}

pub fn set_grid_size(state: NeocivState, xsize: u8, ysize: u8) -> StateResult {
    if state.grid.cells.len() > 0 {
        state_panic!(err_grid_already_defined!());
    } else {
        let mut new_state = state.clone();
        new_state.grid.xsize = xsize;
        new_state.grid.ysize = ysize;

        let cap = (xsize * ysize).into();

        new_state.grid.cells = Vec::with_capacity(cap);

        for i in 0..cap - 1 {
            new_state.grid.cells.push(Cell {
                x: i as u8 % xsize,
                y: i as u8 % (cap / xsize as usize) as u8,
                z: 0,
                owner: None,
                terrain: None,
            });
        }

        return_next_state!(new_state);
    }
}

/// Add a Cell to the grid. This will overwrite any cell that exists at that index.
pub fn add_grid_cell(state: NeocivState, cell: Cell) -> StateResult {
    if cell.owner.is_some() {
        let civ_id = cell.owner.unwrap();
        if invalid_civ_id!(civ_id) {
            state_panic!(err_invalid_civ!(civ_id));
        } else if !civ_id_exists!(state, civ_id) {
            state_panic!(err_unknown_civ!(civ_id));
        }
    }

    if cell.x >= state.grid.xsize {
        state_panic!(err_cell_out_of_bounds!("x", cell.x, state.grid.xsize - 1));
    } else if cell.y >= state.grid.ysize {
        state_panic!(err_cell_out_of_bounds!("y", cell.y, state.grid.ysize - 1));
    } else {
        let mut new_state = state.clone();
        
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
        assert_eq!(
            error.unwrap_err().code,
            crate::errors::StateErrorCode::DuplicateCivId
        );
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
