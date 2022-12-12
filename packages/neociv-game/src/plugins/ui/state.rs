use bevy::prelude::{Component, Resource};

#[derive(Component, Default, Resource)]
pub struct ReplState {
    pub lines: Vec<String>,
    pub input: String,
}

#[derive(Component, Default, Resource)]
pub struct FocusState {
    pub game: bool,
}

#[derive(Component, Default, Resource)]
pub struct NeocivUiState {
    pub focus: FocusState,
    pub repl: ReplState,
}
