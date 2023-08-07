use bevy::prelude::Commands;

use crate::plugins::registry::{self, registry::NeocivRegistry};

// Init with a blank / default registry if it does not already exist
pub fn registry_startup(mut commands: Commands) {
    println!("LOADING THE REGISTRY");
    commands.init_resource::<NeocivRegistry>();
}
