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
