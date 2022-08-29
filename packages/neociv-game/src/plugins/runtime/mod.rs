use bevy::prelude::*;

pub mod startup;
pub mod system;

pub struct NeocivRuntimePlugin;

impl Plugin for NeocivRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup::runtime_startup);
        app.add_system(system::runtime_system);
    }
}