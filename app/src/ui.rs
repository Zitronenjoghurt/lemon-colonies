use crate::ui::widgets::username::UsernameWidget;
use egui_macroquad::egui;
use egui_macroquad::egui::Widget;

pub mod state;
mod widgets;

pub struct UiViewer<'a> {
    pub game: &'a mut crate::game::Game,
    pub http: &'a mut crate::http::Http,
    pub state: &'a mut state::UiState,
    pub toasts: &'a mut egui_notify::Toasts,
}

impl<'a> UiViewer<'a> {
    pub fn show(&mut self, ctx: &egui::Context) {
        self.toasts.show(ctx);

        egui::Window::new("Debug").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Logged in as:");
                UsernameWidget::new(self.http).ui(ui);
            });
            if ui.button("Logout").clicked() {
                self.http.do_logout();
            }
        });
    }
}
