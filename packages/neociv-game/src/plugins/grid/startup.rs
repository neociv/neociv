use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use crate::entities::materials::cell::{CellMaterialContent, CellMaterialEntity};
use crate::plugins::registry::registry::NeocivRegistry;
use crate::utils::hex::*;

/// Setup the hexagonal grid map meshes
pub fn grid_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut registry: ResMut<NeocivRegistry>,
    state: Res<neociv_state::state::NeocivState>,
    asset_server: Res<AssetServer>,
) {
    let size = 1.0;
    let white_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let sq3 = 3_f32.sqrt();

    registry.cell_materials.insert(
        String::from("neociv.contrib.grass"),
        CellMaterialEntity {
            id: String::from("neociv.contrib.grass"),
            content: CellMaterialContent::new(String::from("")),
        },
    );

    state.grid.cells.iter().for_each(|c| {
        // Apply an x offset
        let offset = match c.y % 2 == 0 {
            true => 0.0,
            false => (sq3 * size) / 2.0,
        };

        let x = (c.x as f32) * (sq3 * size) + offset;
        let y = -(c.y as f32) * (size * 1.5);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(hex_mesh(size)),
                //material: registry.cell_materials.get("neociv.contrib.grass").unwrap().content.frag,
                material: white_material.to_owned(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(PickableBundle::default());
    });
}
