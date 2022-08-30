use bevy::prelude::*;
use neociv_civil::runtime::NeocivRuntime;

pub fn runtime_system(mut commands: Commands, mut runtime: ResMut<NeocivRuntime>) {
    runtime.update().unwrap();
    commands.insert_resource(runtime.state.clone());
}
