use bevy::prelude::*;
use neociv_civil::runtime::NeocivRuntime;

pub struct NeocivRuntimePlugin;

impl Plugin for NeocivRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NeocivRuntime::default());
    }
}