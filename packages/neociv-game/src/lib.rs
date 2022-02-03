use bevy::prelude::{App,Plugin};
use bevy::core::CorePlugin;
use bevy::input::InputPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Init,
    Menu,
    Loading,
    Playing,
    Paused,
    Credits,
}

pub struct NeocivGamePlugin;

impl Plugin for NeocivGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup Bevy
            .add_plugin(CorePlugin::default()).add_plugin(InputPlugin::default())
            // Init
            .add_state(GameState::Init)
            // Main Menu
            .add_state(GameState::Menu);
    }
}
