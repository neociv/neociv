use bevy::prelude::*;
use bevy::render::mesh::Indices;

use crate::utils::hex::*;

/// Generates a flat-top hexagonal plane mesh.
fn hex_mesh(size: f32) -> Mesh {
    // Initialise the mesh as a set of vec coordinates, ie. raw geoemetry
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);

    // Positions are 2D vec points with a zero'd z-axis
    let positions = hex_points(size).iter().map(|&p| [p.x, p.y, 0.0]).collect::<Vec<_>>();
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    let mut normals = Vec::with_capacity(6);
    let mut uvs = Vec::with_capacity(6);

    // Normals and UVs are all zero'd
    for _i in 0..7 {
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.0, 0.0]);
    }

    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // Mesh indicies that constantly track back to the two spike x,y and then back to centre
    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1]);
    mesh.set_indices(Some(indices));

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
        mesh: meshes.add(hex_mesh(1.0)),
        material: white_material,
        ..Default::default()
    });
}