use crate::ui::widgets::profile_menu::ProfileMenu;
use crate::ui::widgets::window_button::WindowButton;
use crate::ui::windows::WindowId;
use egui_macroquad::egui;
use egui_macroquad::egui::{TopBottomPanel, Widget};
pub use egui_phosphor::regular as icon;

pub mod state;
mod widgets;
mod windows;

pub struct UiViewer<'a> {
    pub settings: &'a mut crate::settings::Settings,
    pub game: &'a mut crate::game::Game,
    pub http: &'a mut crate::http::Http,
    pub state: &'a mut state::UiState,
    pub toasts: &'a mut egui_notify::Toasts,
    pub ws: &'a mut crate::ws::Ws,
}

impl<'a> UiViewer<'a> {
    pub fn show(&mut self, ctx: &egui::Context) {
        self.toasts.show(ctx);

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.top_panel(ui);
        });

        self.show_windows(ctx);
    }

    fn show_windows(&mut self, ctx: &egui::Context) {
        let window_ids: Vec<WindowId> = self.state.iter_windows().collect();
        for window_id in window_ids {
            window_id.show(self, ctx);
        }
    }

    fn top_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Lemon Colonies");
            ui.separator();
            self.window_button(ui, WindowId::Settings);
            self.window_button(ui, WindowId::Debug);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ProfileMenu::new(&self.game.data, self.http).ui(ui);
            });
        });
    }
}

// Widget helpers
impl<'a> UiViewer<'a> {
    fn window_button(&mut self, ui: &mut egui::Ui, window_id: WindowId) -> egui::Response {
        WindowButton::new(window_id, self.state).ui(ui)
    }
}

pub fn setup_egui(ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::dark());

    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);
}
