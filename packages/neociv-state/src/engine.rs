use crate::civ::{Civ, CivId};
use crate::errors::*;
use crate::state::NeocivState;
use crate::cell::{grid_i_to_xy, grid_xy_to_i, Cell};

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

/// Sets the grid and initialises it with a set of blank cells.
pub fn set_grid_size(state: NeocivState, xsize: u8, ysize: u8) -> StateResult {
    if state.grid.cells.len() > 0 {
        state_panic!(err_grid_already_defined!());
    } else {
        let mut new_state = state.clone();
        new_state.grid.xsize = xsize;
        new_state.grid.ysize = ysize;

        // The maximum number of cells that this grid can contain
        let cap: u16 = (xsize * ysize) as u16;

        new_state.grid.cells = Vec::with_capacity(cap.into());

        for i in 0..cap {
            let xy = grid_i_to_xy(&new_state.grid, i);
            new_state.grid.cells.push(Cell {
                x: xy.0,
                y: xy.1,
                z: 0,
                owner: None,
                terrain: None,
            });
        }

        return_next_state!(new_state);
    }
}

/// Sets the cell at the given x,y in the grid
pub fn set_grid_cell(state: NeocivState, cell: &Cell) -> StateResult {
    if cell.owner.is_some() {
        let civ_id = cell.to_owned().owner.unwrap();
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
        let i = grid_xy_to_i(&new_state.grid, cell.x, cell.y);
        new_state.grid.cells[i as usize] = cell.to_owned();
        return_next_state!(new_state);
    }
}

