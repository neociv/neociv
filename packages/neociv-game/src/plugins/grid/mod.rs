use bevy::prelude::*;

use crate::GameStartupStage;

pub mod utils;
pub mod bundles;
pub mod startup;
pub mod system;

pub struct NeocivGridPlugin;

impl Plugin for NeocivGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage_after(GameStartupStage::Runtime, GameStartupStage::Grid, SystemStage::single_threaded());
        app.add_startup_system_to_stage(GameStartupStage::Grid, startup::grid_startup.label(GameStartupStage::Grid));
    }
}
