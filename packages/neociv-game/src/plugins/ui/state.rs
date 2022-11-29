use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct ReplState {
    pub lines: Vec<String>,
    pub input: String,
}

#[derive(Component, Default)]
pub struct FocusState {
    pub game: bool,
}

#[derive(Component, Default)]
pub struct NeocivUiState {
    pub focus: FocusState,
    pub repl: ReplState,
}
