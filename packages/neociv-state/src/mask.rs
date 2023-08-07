use crate::state_table_default;

/// Cell mask's define an 8-bit value with a bit for each edge condition.
///
/// 0010 0000 (top-left)      / \    0000 0001 (top-right)
///                          /   \
/// 0001 0000 (middle-left) |     |  0000 0010 (middle-right)
///                         |     |
/// 0000 1000 (bottom-left)  \   /   0000 0100 (bottom-right)
///                           \ /
pub type CellMask = u8;

state_table_default! {
    pub struct CellMasks {
        /// Whether or not the neighbour shares the same owner
        pub owner: CellMask,
        /// Whether or not the neighbour is the same terrain type
        pub terrain: CellMask,
        /// Whether or not the neighbour is water while the cell is ground
        pub coast: CellMask,
        /// Whether or not the neighbour has a road (or equivalent) on it
        pub road: CellMask,
    }
}
