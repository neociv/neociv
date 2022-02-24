use crate::cell::Grid;
use crate::civ::Civ;

#[derive(Clone, Default)]
pub struct NeocivState {
    pub civs: Vec<Civ>,
    pub grid: Grid,
}
