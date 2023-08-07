use bevy::prelude::{IntoSystemDescriptor, Plugin, StartupStage, SystemStage};

use crate::GameStartupStage;

pub mod registry;
pub mod startup;

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_stage_after(
            StartupStage::PostStartup,
            GameStartupStage::Registry,
            SystemStage::single_threaded(),
        );
        app.add_startup_system_to_stage(
            GameStartupStage::Registry,
            startup::registry_startup.label(GameStartupStage::Registry),
        );
    }
}
