use crate::civ::{CivId};

/// Representation of a single Cell in the Grid.
#[derive(Clone)]
pub struct Cell {
    /// Horizontal (East) position of the cell.
    pub x: u8,
    /// Vertical (North) position of the cell.
    pub y: u8,
    /// Elevation of the cell.
    pub z: u8,
    /// Civ that owns this cell, optional.
    pub owner: Option<CivId>,
}

/// Contains Cells in a 1D Vec that are addressable in 2D space according to the xsize / ysize.
#[derive(Clone,Default)]
pub struct Grid {
    pub xsize: u8,
    pub ysize: u8,
    pub cells: Vec::<Cell>,
}
