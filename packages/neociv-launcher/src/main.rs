#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, Frame};
use egui::CentralPanel;
use egui_dock::{DockArea, TabViewer};
use tabs::LauncherTabs;

pub mod tabs;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Neociv Launcher",
        options,
        Box::new(|_cc| Box::<LauncherApp>::default()),
    )
}

struct LauncherApp {
    logo: egui_extras::RetainedImage,
    tabs: LauncherTabs,
}

impl Default for LauncherApp {
    fn default() -> Self {
        Self {
            logo: egui_extras::RetainedImage::from_svg_bytes_with_size(
                "logo.svg",
                include_bytes!("../../../logo.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
            tabs: LauncherTabs::new(),
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let max_size = ui.available_size();

            // Draw logo
            self.logo.show_size(ui, egui::vec2(770.0, 180.0));
            ui.end_row();

            ui.spacing();

            // Draw tabs
            DockArea::new(&mut self.tabs.tree)
                .draggable_tabs(false)
                .show_close_buttons(false)
                .show_add_buttons(false)
                .show_inside(ui, &mut tabs::TabViewer {});
            ui.end_row();
        });
    }
}
