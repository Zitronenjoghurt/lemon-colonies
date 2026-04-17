use crate::ui::state::UiState;
use crate::ui::windows::WindowId;
use egui_macroquad::egui::{Response, Ui, Widget};

pub struct WindowButton<'a> {
    pub window_id: WindowId,
    pub state: &'a mut UiState,
}

impl<'a> WindowButton<'a> {
    pub fn new(window_id: WindowId, state: &'a mut UiState) -> Self {
        Self { window_id, state }
    }
}

impl Widget for WindowButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let is_open = self.state.is_window_open(self.window_id);
        let response = ui
            .selectable_label(is_open, self.window_id.icon())
            .on_hover_text(self.window_id.title());
        if response.clicked() {
            self.state.toggle_window(self.window_id);
        }
        response
    }
}
