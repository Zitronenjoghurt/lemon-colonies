use crate::ui::widgets::settings::SettingsWidget;
use crate::ui::UiViewer;
use egui_macroquad::egui;
use egui_macroquad::egui::Widget;

pub fn show(v: &mut UiViewer, ui: &mut egui::Ui) {
    SettingsWidget::new(v.settings).ui(ui);
}
