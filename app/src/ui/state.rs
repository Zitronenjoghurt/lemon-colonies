use crate::storage::Storage;
use crate::ui::windows::WindowId;
use std::collections::HashSet;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct UiState {
    windows: HashSet<WindowId>,
    #[serde(skip, default)]
    last_save: f64,
}

impl UiState {
    pub fn load() -> Self {
        let state_string = Storage::get("ui_state");
        state_string.map_or_else(UiState::default, |s| {
            serde_json::from_str(&s).unwrap_or_default()
        })
    }

    pub fn save(&self) {
        let Ok(state_string) = serde_json::to_string(self) else {
            return;
        };
        Storage::set("ui_state", &state_string);
    }

    pub fn update(&mut self) {
        let now = egui_macroquad::macroquad::time::get_time();
        if now - self.last_save > 30.0 {
            self.save();
            self.last_save = now;
        }
    }

    pub fn toggle_window(&mut self, window: WindowId) {
        if self.windows.contains(&window) {
            self.windows.remove(&window);
        } else {
            self.windows.insert(window);
        }
    }

    pub fn is_window_open(&self, window: WindowId) -> bool {
        self.windows.contains(&window)
    }

    pub fn iter_windows(&self) -> impl Iterator<Item = WindowId> {
        self.windows.iter().copied()
    }
}
