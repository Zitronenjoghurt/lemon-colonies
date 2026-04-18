use crate::ui::{icon, UiViewer};
use egui_macroquad::egui;
use egui_macroquad::egui::Id;
use strum_macros::EnumIter;

mod debug;
mod settings;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, EnumIter,
)]
pub enum WindowId {
    Debug,
    Settings,
}

impl WindowId {
    pub fn show(&self, viewer: &mut UiViewer, ctx: &egui::Context) {
        let mut open = true;
        egui::Window::new(self.title())
            .id(Id::new(self))
            .open(&mut open)
            .collapsible(self.collapsible())
            .show(ctx, |ui| match self {
                WindowId::Debug => debug::show(viewer, ui),
                WindowId::Settings => settings::show(viewer, ui),
            });
        if !open {
            viewer.state.toggle_window(*self);
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            WindowId::Debug => "Debug",
            WindowId::Settings => "Settings",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            WindowId::Debug => icon::BUG,
            WindowId::Settings => icon::GEAR,
        }
    }

    pub fn collapsible(&self) -> bool {
        false
    }
}
