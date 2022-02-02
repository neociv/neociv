use crate::civ::Civ;
use crate::cell::Grid;

#[derive(Clone,Default)]
pub struct NeocivState {
    pub civs: Vec::<Civ>,
    pub grid: Grid,
}
