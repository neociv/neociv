use bevy::prelude::{App, Plugin, StageLabel, SystemLabel};
use bevy_mod_picking::{DebugCursorPickingPlugin, DebugEventsPickingPlugin, DefaultPickingPlugins};

pub mod grid;
pub mod plugins;
pub mod utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Loading,
    Playing,
    Paused,
    Credits,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel, SystemLabel)]
pub enum GameStartupStage {
    Registry = 0,
    Runtime = 1,
    UI = 2,
    Grid = 3,
    Picking = 4,
    Camera = 5,
}

pub struct NeocivGamePlugin;

impl Plugin for NeocivGamePlugin {
    fn build(&self, app: &mut App) {
        // Add the registry plugin - this must exist first
        app.add_plugin(plugins::registry::RegistryPlugin);

        // Add the runtime to the resources
        app.add_plugin(plugins::runtime::NeocivRuntimePlugin);

        // Grid plugin
        app.add_plugin(plugins::grid::NeocivGridPlugin);

        // Setup the UI
        app.add_plugin(plugins::ui::NeocivUiPlugin);

        // Picking
        app.add_plugins(DefaultPickingPlugins)
            .add_plugin(DebugCursorPickingPlugin)
            .add_plugin(DebugEventsPickingPlugin);

        // Normal
        app.add_startup_system(grid::camera::setup_grid_camera)
            .add_startup_system(grid::map::setup_grid_map)
            .add_startup_system(grid::light::setup_grid_lights);

        // Systems
        app.add_system(grid::camera::grid_camera_system);
    }
}
