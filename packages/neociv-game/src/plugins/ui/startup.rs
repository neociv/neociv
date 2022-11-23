use bevy::prelude::*;

use neociv_state::state::NeocivState;
use super::state::NeocivUiState;

pub fn ui_startup(mut commands: Commands, state: Res<NeocivState>) {
    commands.insert_resource(NeocivUiState::default());
}
