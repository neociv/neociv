use bevy::prelude::Plugin;

pub mod registry;
pub mod startup;

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(startup::registry_startup);
    }
}