use bevy::prelude::*;
use neociv_civil::runtime::NeocivRuntime;

pub fn runtime_system(mut commands: Commands, runtime: Res<NeocivRuntime>) {
    commands.insert_resource(runtime.state);
}
