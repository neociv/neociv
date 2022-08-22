use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;

pub fn setup_grid_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());
}

pub fn grid_camera_system (time: Res<Time>, mut query: Query<(&mut Camera3d, &mut Transform)>, state: Res<neociv_state::state::NeocivState>) {
	for (mut camera, mut transform) in query.iter_mut() {
        // TODO: Remove this, obviously
        transform.translation = state.camera.position + Vec3::new(0f32, 0f32, 30f32);
    }
}