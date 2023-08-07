use std::f32::consts::PI;

use bevy::prelude::*;

#[macro_export]
macro_rules! hex_idx {
    () => {
        bevy::render::mesh::Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1])
    };
}
pub(crate) use hex_idx;

#[macro_export]
macro_rules! hex_pos {
    ($vecs:expr) => {
        $vecs.iter().map(|&p| [p.x, p.y, 0.0]).collect::<Vec<_>>()
    };
}
pub(crate) use hex_pos;

/// Calculate pointy top hexagon x,y points as Vec2
pub fn hex_points(size: f32) -> Vec<Vec2> {
    let angle_deg = |i: f32| (60.0 * i - 30.0);
    let angle_rad = |i: f32| (PI / 180.0 * angle_deg(i));
    let x = |i: f32| (size * angle_rad(i).cos());
    let y = |i: f32| (size * angle_rad(i).sin());
    let mut positions = Vec::with_capacity(6);

    positions.push(Vec2::new(0.0, 0.0));

    for i in 0..6 {
        positions.push(Vec2::new(x(i as f32), y(i as f32)));
    }

    return positions;
}

/// Generates a flat-top hexagonal plane mesh.
pub fn hex_mesh(size: f32) -> Mesh {
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
