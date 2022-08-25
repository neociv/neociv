use bevy::prelude::*;
use neociv_civil::runtime::NeocivRuntime;

pub fn runtime_startup(mut commands: Commands) {
    // Insert the runtime and start it up so that it is available for later modification.
    // This is intended to be the blank slate that is acted upon.
    commands.insert_resource(NeocivRuntime::default());
}