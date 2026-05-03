use crate::fps_counter::FpsCounter;
use crate::ui::widgets::window_button::WindowButton;
use crate::ui::windows::WindowId;
use egui_macroquad::egui;
use egui_macroquad::egui::Widget;
pub use egui_phosphor::regular as phosphor;

pub mod icon;
pub mod panels;
pub mod state;
mod widgets;
pub mod windows;

pub struct UiViewer<'a> {
    pub settings: &'a mut crate::settings::Settings,
    pub fps_counter: FpsCounter,
    pub game: &'a mut crate::game::Game,
    pub http: &'a mut crate::http::Http,
    pub icons: &'a icon::IconCache,
    pub server_time: &'a crate::server_time::ServerTime,
    pub state: &'a mut state::UiState,
    pub toasts: &'a mut egui_notify::Toasts,
    pub ws: &'a mut crate::ws::Ws,
}

impl<'a> UiViewer<'a> {
    pub fn show(&mut self, ctx: &egui::Context) {
        self.toasts.show(ctx);

        panels::meta::show(ctx, self);
        panels::info::show(ctx, self);
        panels::action::show(ctx, self);
        panels::sub_action::show(ctx, self);
        panels::hover::show(ctx, self);

        self.show_windows(ctx);
    }

    fn show_windows(&mut self, ctx: &egui::Context) {
        let window_ids: Vec<WindowId> = self.state.iter_windows().collect();
        for window_id in window_ids {
            window_id.show(self, ctx);
        }
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
