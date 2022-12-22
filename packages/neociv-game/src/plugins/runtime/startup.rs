use bevy::prelude::*;
use neociv_civil::runtime::NeocivRuntime;
use neociv_state::state::NeocivState;

/// Insert the runtime and start it up so that it is available for later modification. This is
/// intended to be the blank slate that is acted upon.
pub fn runtime_startup(mut commands: Commands, world_q: Query<&NeocivState>) {
    println!("LOADING THE RUNTIME");
    if true || world_q.is_empty() {
        // Init the resource as empty and await any potential states
        commands.init_resource::<NeocivRuntime>();
    } else {
        // Insert the resource with the provided state
        let s = world_q.single();
        let mut r = NeocivRuntime::default();
        match r.inject_state(s) {
            Ok(r) => commands.insert_resource(r.clone()),
            _ => panic!("Cannot insert runtime"),
        }
    }
}
