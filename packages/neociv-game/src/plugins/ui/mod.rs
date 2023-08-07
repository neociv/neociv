use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::GameStartupStage;

pub mod startup;
pub mod state;
pub mod system;

pub struct NeocivUiPlugin;

impl Plugin for NeocivUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);

        app.add_startup_stage_after(
            GameStartupStage::Grid,
            GameStartupStage::UI,
            SystemStage::single_threaded(),
        );
        app.add_startup_system_to_stage(
            GameStartupStage::UI,
            startup::ui_startup.label(GameStartupStage::UI),
        );

        app.add_system(system::ui_system);
    }
}
