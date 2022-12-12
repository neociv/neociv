use crate::{civ::CivKey, mask::CellMasks, state_enum_default, state_table_default};

use self::improvement::Improvement;

pub mod improvement;

state_enum_default! {
    pub enum Terrain {
        DeepWater,
        Water,
        #[default]
        Ground,
    }
}

state_table_default! {
    /// Representation of a single Cell in the Grid.
    pub struct Cell {
        /// Horizontal (East) position of the cell.
        pub x: u8,
        /// Vertical (North) position of the cell.
        pub y: u8,
        /// Civ that owns this cell, optional.
        pub owner: Option<CivKey>,
        /// Terrain
        pub terrain: Terrain,
        /// Masks
        pub masks: CellMasks,
        /// Improvement
        pub improvement: Option<Improvement>,
    }
}
