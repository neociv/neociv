use bevy::prelude::*;

#[derive(Bundle)]
pub struct CellBundle {
    pub owner_mask: u8,
    pub coast_mask: u8,
    pub x: u8,
    pub y: u8,
    pub mesh: Handle<Mesh>,
    // TODO: Improvement... technically a different bundle?
    // TODO: ResourceMaterial
    // TODO: TerrainMaterial
}

impl Default for CellBundle {
    fn default() {
        CellBundle {
            owner_mask: 0,
            coast_mask: 0,
            x: 0,
            y: 0,
            mesh: crate::plugins::grid::utils::hex_mesh(1),
        }
    }
}
