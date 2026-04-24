use crate::ui::widgets::debug::DebugWidget;
use crate::ui::UiViewer;
use egui_macroquad::egui;
use egui_macroquad::egui::Widget;

pub fn show(v: &mut UiViewer, ui: &mut egui::Ui) {
    DebugWidget::new(v.game, v.settings).ui(ui);
}
