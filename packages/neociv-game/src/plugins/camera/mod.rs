use crate::GameStartupStage;
use bevy::prelude::*;

pub mod startup;
pub mod system;

pub struct NeocivCameraPlugin;

impl Plugin for NeocivCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage_after(
            GameStartupStage::Grid,
            GameStartupStage::Camera,
            SystemStage::single_threaded(),
        );
        app.add_startup_system_to_stage(
            GameStartupStage::Camera,
            startup::setup_grid_camera.label(GameStartupStage::Grid),
        );
        app.add_system(system::grid_camera_system);
    }
}
