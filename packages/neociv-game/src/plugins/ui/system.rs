use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use neociv_state::state::NeocivState;

pub fn ui_system(state: Res<NeocivState>, mut egui_context: ResMut<EguiContext>) {
    // Create the debug window
    egui::Window::new(state.revision.to_string()).show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });

    // TODO: Create the topbar

    // TODO: Create the bottom bar
    // TODO: Create the mini-map

    // TODO: Create the left docked frame with trees

    // TODO: Create the right docked frame with focus and info screens
}
