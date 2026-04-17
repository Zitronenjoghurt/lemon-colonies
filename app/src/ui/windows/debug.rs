use crate::ui::widgets::connection_status::ConnectionStatusWidget;
use crate::ui::widgets::username::UsernameWidget;
use crate::ui::UiViewer;
use egui_macroquad::egui;
use egui_macroquad::egui::Widget;

pub fn show(v: &mut UiViewer, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Loaded chunks:");
        ui.label(v.game.world.chunk_count().to_string());
    });

    ui.horizontal(|ui| {
        ui.label("Logged in as:");
        UsernameWidget::new(v.http).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("WebSocket:");
        ConnectionStatusWidget::new(v.ws).ui(ui);
        if ui.button("Hello").clicked() {
            v.ws.hello();
        }
    });

    if ui.button("Logout").clicked() {
        v.http.do_logout();
    }
}
