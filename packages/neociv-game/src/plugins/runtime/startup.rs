use bevy::prelude::*;
use neociv_civil::runtime::NeocivRuntime;

/// Insert the runtime and start it up so that it is available for later modification. This is
/// intended to be the blank slate that is acted upon.
pub fn runtime_startup(mut commands: Commands) {
    // Create and initialise the runtime, this will generate the callback to also insert the state when it changes
    commands.insert_resource(NeocivRuntime::default());
}
