use crate::utils::hex::*;
use bevy::prelude::*;

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
