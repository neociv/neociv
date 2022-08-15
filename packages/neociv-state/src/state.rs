use std::collections::HashMap;

use crate::camera::Camera;
use crate::cell::Grid;
use crate::civ::{Civ, CivKey};

/// Game state structure
#[derive(Clone, Default, Debug)]
pub struct NeocivState {
    /// The current change revision of the state - all actions that change the state will increment
    /// this value which is useful for quickly checking if the state has changed and needs to be
    /// inspected again.
    pub revision: u64,
    /// The turn currently being played - this is intended to tick over when the first Civ begins
    /// playing. Games that are in turn "0" should be considered in an initialisation state.
    pub turn: u32,
    /// Key of the currently active Civ
    pub active_civ_key: Option<CivKey>,
    /// Order of civs to play through which removes inactive civs and avoids issues with ordering
    /// between serialized formats.
    pub civ_order: Vec<CivKey>,
    /// Directory of all Civs
    pub civs: HashMap<CivKey, Civ>,
    /// The grid of cells that make up the game map.
    pub grid: Grid,
    /// The current camera state
    pub camera: Camera,
}
