use crate::ui::windows::WindowId;
use std::collections::HashSet;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct UiState {
    windows: HashSet<WindowId>,
}

impl UiState {
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
