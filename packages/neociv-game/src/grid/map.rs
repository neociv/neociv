use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::mesh::Indices;

/// Generates a flat-top hexagonal plane mesh.
fn hex_mesh(size: u8) -> Mesh {

    let x = |i: f32| (i * 2.0 * PI / 6.0).cos();
    let y = |i: f32| (i * 2.0 * PI / 6.0).sin();

    let mut positions = Vec::with_capacity(18);


    // Centre
    for i in 0..6 {
        positions.push([0.0, 0.0, 0.0]);
        positions.push([x(i as f32), y(i as f32), 0.0]);
        positions.push([x(i as f32 + 1.0), y(i as f32 + 1.0), 0.0]);
    }

    let indicies = Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1]);
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(indicies));

    return mesh;
}

/// Setup the hexagonal grid map meshes
pub fn setup_grid_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //let mesh = meshes.add(Mesh::from(shape::Plane { size: 5.0 }));
    let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(hex_mesh(1)),
        material: white_material,
        ..Default::default()
    });
}
