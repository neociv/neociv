use egui_dock::{NodeIndex, Style, Tree};

pub struct LauncherTabs {
    pub tree: Tree<String>,
}

pub struct TabViewer;

impl LauncherTabs {
    pub fn new() -> Self {
        let tab1 = "Game".to_string();
        let tab2 = "Settings".to_string();
        let tab3 = "Mods".to_string();

        let mut tree = Tree::new(vec![tab1, tab2, tab3]);

        Self { tree }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui_dock::DockArea::new(&mut self.tree)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut TabViewer {})
    }
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}
