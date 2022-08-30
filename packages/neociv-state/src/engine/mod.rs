use bevy_math::Vec3;

use crate::cell::{Cell};
use crate::grid::{grid_i_to_xy, grid_xy_to_i};
use crate::civ::{Civ, CivKey};
use crate::errors::*;
use crate::mask::CellMasks;
use crate::state::NeocivState;

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
#[macro_export]
macro_rules! state_panic {
    ($err:expr) => {
        return Err($err);
    };
}
pub(crate) use state_panic;

/// Validate the CivId
macro_rules! invalid_civ_id {
    ($id:expr) => {
        !crate::civ::VALID_CIV_ID.is_match($id)
    };
}

/// Validate the CivKey
macro_rules! invalid_civ_key {
    ($key:expr) => {
        !crate::civ::VALID_CIV_KEY.is_match($key)
    };
}

/// Whether or not the CivKey exists
macro_rules! civ_key_exists {
    ($state:expr, $key:expr) => {
        $state.civs.contains_key($key)
    };
}

/// Initialise an empty & default state.
pub fn init() -> NeocivState {
    return NeocivState::default();
}

/// Add a Civ to the state.
pub fn add_civ(state: NeocivState, civ: Civ) -> StateResult {
    if invalid_civ_id!(&civ.id) {
        state_panic!(err_invalid_civ_id!(civ.id));
    } else {
        let mut new_state = state.clone();
        let idx = state.civs.iter().fold(0, |accum, item| {
            accum + if item.1.id == civ.id { 1 } else { 0 }
        });

        // Generate the civ_key as a combination of the CivId and the 0-based index
        let civ_key = format!("{}[{}]", civ.id, idx);

        // Add to the order
        new_state.civ_order.push(civ_key.to_string());

        // Add to the map of Civs
        new_state.civs.insert(civ_key, civ);
        return_next_state!(new_state);
    }
}

/// Remove a Civ from the state - including from all ownership roles and owned units. This will
/// never naturally be used by the game however it is useful to retain.
pub fn remove_civ(state: NeocivState, civ_key: CivKey) -> StateResult {
    if invalid_civ_key!(&civ_key) {
        state_panic!(err_invalid_civ_key!(civ_key));
    } else {
        let mut new_state = state.clone();
        // Remove from civ order
        new_state.civ_order.retain(|ck| ck != &civ_key);
        // Iterate over cells to remove
        for c in new_state.grid.cells.iter_mut() {
            // TODO: Remove units
            // TODO: Remove improvements?
            // Remove ownership of any owned cells
            if c.owner.as_ref().unwrap() == &civ_key {
                c.owner = None;
            }
        }
        // Remove from the list of civs
        new_state.civs.remove(&civ_key);
        return_next_state!(new_state);
    }
}

/// Sets the grid and initialises it with a set of blank cells. Calling this will effectively
/// prevent it from being called again on the state.
pub fn set_grid_size(state: NeocivState, xsize: u8, ysize: u8) -> StateResult {
    if state.grid.cells.len() > 0 {
        state_panic!(err_grid_already_defined!());
    } else {
        let mut new_state = state.clone();
        new_state.grid.xsize = xsize;
        new_state.grid.ysize = ysize;

        // The maximum number of cells that this grid can contain
        let cap: u16 = xsize as u16 * ysize as u16;

        new_state.grid.cells = Vec::with_capacity(cap.into());

        for i in 0..cap {
            let xy = grid_i_to_xy(&new_state.grid, i);
            new_state.grid.cells.push(Cell {
                x: xy.0,
                y: xy.1,
                owner: None,
                terrain: None,
                masks: CellMasks::default(),
            });
        }

        return_next_state!(new_state);
    }
}

/// Sets the cell at the given x,y in the grid
pub fn set_grid_cell(state: NeocivState, cell: &Cell) -> StateResult {
    if cell.owner.is_some() {
        let civ_key = cell.to_owned().owner.unwrap();
        if invalid_civ_key!(&civ_key) {
            state_panic!(err_invalid_civ_key!(civ_key));
        } else if !civ_key_exists!(state, &civ_key) {
            state_panic!(err_unknown_civ_key!(civ_key));
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

/// Modify the camera's position with relative values
pub fn mod_camera_position(state: NeocivState, vector: Vec3) -> StateResult {
   let mut new_state = state.clone(); 
   new_state.camera.position += vector;
   return_next_state!(new_state);
}
