use bevy::prelude::{App, Plugin};

pub mod utils;
pub mod grid;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Loading,
    Playing,
    Paused,
    Credits,
}

pub struct NeocivGamePlugin;

impl Plugin for NeocivGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(grid::camera::setup_grid_camera)
            .add_startup_system(grid::map::setup_grid_map)
            .add_startup_system(grid::light::setup_grid_lights);
    }
}
