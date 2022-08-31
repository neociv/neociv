use bevy::prelude::*;
use neociv_civil::runtime::NeocivRuntime;

pub fn runtime_system(mut commands: Commands, mut runtime: ResMut<NeocivRuntime>) {
    let _result = runtime.update();
    commands.insert_resource(runtime.get_state().unwrap().clone());
}
