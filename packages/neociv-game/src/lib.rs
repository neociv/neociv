use bevy::prelude::{App,Plugin};
use bevy::core::CorePlugin;
use bevy::input::InputPlugin;

pub struct NeocivGamePlugin;

impl Plugin for NeocivGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CorePlugin::default()).add_plugin(InputPlugin::default());
    }
}
