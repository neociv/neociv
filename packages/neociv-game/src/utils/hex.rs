use std::f32::consts::PI;

use bevy::prelude::*;

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
