use crate::ui::UiViewer;
use egui_macroquad::egui;
use egui_macroquad::egui::TopBottomPanel;

pub fn show(ctx: &egui::Context, v: &mut UiViewer) {
    TopBottomPanel::new(v.settings.action_panel_pos.egui(), "action_panel").show(ctx, |ui| {
        ui.label("Actions");
    });
}
