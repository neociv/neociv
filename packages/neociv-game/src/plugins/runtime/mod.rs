use bevy::prelude::*;

use crate::GameStartupStage;

pub mod startup;
pub mod system;

pub struct NeocivRuntimePlugin;

impl Plugin for NeocivRuntimePlugin {
    fn build(&self, app: &mut App) {
        // Startup - the registry *must* be available before the runtime initliases
        app.add_startup_stage_after(GameStartupStage::Registry, GameStartupStage::Runtime, SystemStage::single_threaded());
        app.add_startup_system_to_stage(GameStartupStage::Runtime, startup::runtime_startup.label(GameStartupStage::Runtime));

        // System
        app.add_system(system::runtime_system);
    }
}