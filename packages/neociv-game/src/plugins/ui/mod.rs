use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod state;
pub mod startup;
pub mod system;

pub struct NeocivUiPlugin;

impl Plugin for NeocivUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_startup_system(startup::ui_startup);
        app.add_system(system::ui_system);
    }
}

