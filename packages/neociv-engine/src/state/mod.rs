use neociv_db::NeocivDB;
use neociv_state::errors::{StateError, StateErrorCode};
use neociv_state::state::NeocivState;
use neociv_state::{err_unknown_state_prop, state_error};

pub mod prop_updates;

use crate::state::prop_updates::*;

pub type StateUpdateResult = Result<NeocivState, StateError>;

/// Update the *entire* state.
#[inline]
pub fn update_state(current: &NeocivState, db: &NeocivDB) -> StateUpdateResult {
    update_state_prop("*", current, db)
}

/// Update a specific property on the state, passing in "*" will update *all* properties.
#[inline]
pub fn update_state_prop(prop: &str, current: &NeocivState, db: &NeocivDB) -> StateUpdateResult {
    match prop {
        "*" => update_state_props(vec!["grid"], current, db),
        &_ => update_state_props(vec![prop], current, db),
    }
}

/// Update a set of specific properties. Does _not_ support passing in "*" for *all* properties
#[inline]
pub fn update_state_props(
    props: Vec<&str>,
    current: &NeocivState,
    db: &NeocivDB,
) -> StateUpdateResult {
    let mut state = current.clone();

    for prop in props {
        match prop {
            "grid" => state.grid = update_grid(db),
            "grid.cells" => state.grid.cells = update_grid_cells(db),
            &_ => return Err(err_unknown_state_prop!(prop)),
        }
    }

    return Ok(state);
}
