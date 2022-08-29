use bevy::prelude::*;
use neociv_state::mask::CellMasks;

#[derive(Bundle)]
pub struct CellBundle {
    pub x: u8,
    pub y: u8,
    pub mesh: Handle<Mesh>,
    pub masks: CellMasks,
    // TODO: Improvement... technically a different bundle?
    // TODO: ResourceMaterial
    // TODO: TerrainMaterial
}

impl Default for CellBundle {
    fn default() {
        Self {
            x: 0,
            y: 0,
            mesh: crate::plugins::grid::utils::hex_mesh(1),
            masks: CellMasks::default(),
        }
    }
}
