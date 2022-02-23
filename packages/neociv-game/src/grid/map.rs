use bevy::prelude::*;

pub fn setup_grid_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 5.0 }));
    let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        material: white_material,
        ..Default::default()
    });
}
