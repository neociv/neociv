use bevy::prelude::Plugin;

pub mod loader;
pub mod startup;

pub struct ContentLoaderPlugin;

impl Plugin for ContentLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(startup::loader_startup);
    }
}