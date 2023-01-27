use bevy::prelude::*;

pub fn grid_camera_system(
    time: Res<Time>,
    mut query: Query<(&mut Camera3d, &mut Transform)>,
    state: Res<neociv_state::state::NeocivState>,
) {
    for (mut camera, mut transform) in query.iter_mut() {
        transform.translation = state.camera.position + Vec3::new(0f32, 0f32, 30f32);
    }
}
