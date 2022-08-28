use bevy::prelude::*;

pub mod utils;
pub mod bundles;
pub mod startup;

pub struct NeocivGridPlugin;

impl Plugin for NeocivGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup::grid_startup);
    }
}
