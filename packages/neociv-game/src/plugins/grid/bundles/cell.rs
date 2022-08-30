use bevy::prelude::*;
use neociv_state::mask::CellMasks;

#[derive(Component)]
pub struct CellBundle {
    pub x: u8,
    pub y: u8,
    //pub mesh: Handle<Mesh>,
    pub mesh: Mesh,
    pub masks: CellMasks,
    // TODO: Improvement... technically a different bundle?
    // TODO: ResourceMaterial
    // TODO: TerrainMaterial
}

impl Default for CellBundle {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            mesh: crate::plugins::grid::utils::hex_mesh(1.0),
            masks: CellMasks::default(),
        }
    }
}
