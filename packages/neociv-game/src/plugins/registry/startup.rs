use bevy::prelude::Commands;

use crate::plugins::registry::registry::NeocivRegistry;

// Init with a blank / default registry if it does not already exist
pub fn registry_startup(mut commands: Commands) {
    commands.init_resource::<NeocivRegistry>();
}