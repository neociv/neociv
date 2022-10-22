use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub fn ui_system(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}