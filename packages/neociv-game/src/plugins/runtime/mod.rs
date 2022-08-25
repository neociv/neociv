use bevy::prelude::*;
use neociv_civil::runtime::NeocivRuntime;

pub mod startup;

pub struct NeocivRuntimePlugin;

impl Plugin for NeocivRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup::runtime_startup);
    }
}