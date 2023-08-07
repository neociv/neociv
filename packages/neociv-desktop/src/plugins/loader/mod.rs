use bevy::prelude::*;

use neociv_game::GameStartupStage;

pub mod loader;
pub mod startup;

pub struct ContentLoaderPlugin;

#[derive(StageLabel, SystemLabel)]
enum DesktopStage {
    Loader = 9,
}

impl Plugin for ContentLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_stage_after(
            GameStartupStage::Runtime,
            DesktopStage::Loader,
            SystemStage::single_threaded(),
        );
        app.add_startup_system_to_stage(
            DesktopStage::Loader,
            startup::loader_startup.label(DesktopStage::Loader),
        );
    }
}
