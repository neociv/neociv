use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct ReplState {
    pub lines: Vec<String>,
    pub input: String,
}

#[derive(Component, Default)]
pub struct NeocivUiState {
    pub repl: ReplState,
}
