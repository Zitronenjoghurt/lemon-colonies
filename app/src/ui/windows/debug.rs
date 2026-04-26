use crate::ui::widgets::debug::DebugWidget;
use crate::ui::UiViewer;
use egui_macroquad::egui;
use egui_macroquad::egui::Widget;

pub fn show(v: &mut UiViewer, ui: &mut egui::Ui) {
    DebugWidget::new(v.fps_counter, v.game, v.settings, v.server_time).ui(ui);
}
