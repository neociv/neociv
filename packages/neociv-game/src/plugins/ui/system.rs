use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_egui::{egui, EguiContext};

use neociv_civil::runtime::NeocivRuntime;
use neociv_state::state::NeocivState;

use super::state::NeocivUiState;

pub fn ui_system(
    state: Res<NeocivState>,
    mut egui_context: ResMut<EguiContext>,
    runtime: Res<NeocivRuntime>,
    mut ui_state: ResMut<NeocivUiState>,
    keys: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    // Create the debug window
    egui::Window::new(state.revision.to_string()).show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });

    // Top Bar
    egui::TopBottomPanel::top("panel.top").show(egui_context.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });

    // Bottom Bar
    egui::TopBottomPanel::bottom("panel.bottom").show(egui_context.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });

    // TODO: Create the mini-map

    // TODO: Create the left docked frame with trees

    // TODO: Create the right docked frame with focus and info screens

    // TODO: Make toggleable
    egui::Window::new("REPL").show(egui_context.ctx_mut(), |ui| {
        ui_state.repl.lines.iter().for_each(|line| {
            let lbl_response = ui.add(egui::Label::new(line));
            lbl_response.surrender_focus();
        });

        let response = ui.add(egui::TextEdit::singleline(&mut ui_state.repl.input));

        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
            // Clone and store the input value
            let input = ui_state.repl.input.clone();

            // CLear the input from the screen
            ui_state.repl.input.clear();

            match runtime.eval_lua::<String>(input.as_str()) {
                Ok(s) => {
                    ui_state.repl.lines.push(input);
                    ui_state.repl.lines.push(s);
                }
                Err(ex) => {
                    ui_state.repl.lines.push(ex.to_string());
                }
            }

            // Refocus - the enter key removes the focus, we want to regain it so we can continue typing
            response.request_focus();
        }
    });

    ui_state.focus.game = egui_context.ctx_mut().memory().focus().is_none();

    if ui_state.focus.game {
        if keys.pressed(KeyCode::W) {
            runtime.eval_lua::<()>(r#"cvl.dispatch("camera.move.up")"#);
        }
        if keys.pressed(KeyCode::S) {
            runtime.eval_lua::<()>(r#"cvl.dispatch("camera.move.down")"#);
        }
        if keys.pressed(KeyCode::A) {
            runtime.eval_lua::<()>(r#"cvl.dispatch("camera.move.left")"#);
        }
        if keys.pressed(KeyCode::D) {
            runtime.eval_lua::<()>(r#"cvl.dispatch("camera.move.right")"#);
        }
        for ev in scroll_evr.iter() {
            if ev.y > 0.0 {
                runtime.eval_lua::<()>(r#"cvl.dispatch("camera.zoom.in")"#);
            } else if ev.y < 0.0 {
                runtime.eval_lua::<()>(r#"cvl.dispatch("camera.zoom.out")"#);
            }
        }
    }
}
