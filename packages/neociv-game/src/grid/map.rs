use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy_mod_picking::PickableBundle;

use crate::utils::hex::*;

/// Generates a flat-top hexagonal plane mesh.
fn hex_mesh(size: f32) -> Mesh {
    // Initialise the mesh as a set of vec coordinates, ie. raw geoemetry
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);

    // Positions are 2D vec points with a zero'd z-axis
    let positions = hex_pos!(hex_points(size));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    let mut normals = Vec::with_capacity(6);
    let mut uvs = Vec::with_capacity(6);

    // Normals and UVs are all zero'd
    for _i in 0..7 {
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.0, 0.0]);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // Mesh indicies that constantly track back to the two spike x,y and then back to centre
    mesh.set_indices(Some(hex_idx!()));

    return mesh;
}

/// Setup the hexagonal grid map meshes
pub fn setup_grid_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<neociv_state::state::NeocivState>,
) {
    let size = 1.0;
    let white_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let sq3 = 3_f32.sqrt();

    state.grid.cells.iter().for_each(|c| {
        // Apply an x offset
        let offset = match c.y % 2 == 0 {
            true => 0.0,
            false => (sq3 * size) / 2.0,
        };

        let x = (c.x as f32) * (sq3 * size) + offset;
        let y = -(c.y as f32) * (size * 1.5);

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(hex_mesh(size)),
                material: white_material.to_owned(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert_bundle(PickableBundle::default());
    });
}
