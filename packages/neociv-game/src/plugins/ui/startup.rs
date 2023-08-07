use bevy::prelude::*;

use super::state::NeocivUiState;
use neociv_state::state::NeocivState;

pub fn ui_startup(mut commands: Commands, state: Res<NeocivState>) {
    commands.insert_resource(NeocivUiState::default());
}
