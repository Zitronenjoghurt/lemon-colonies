use crate::storage::Storage;
use egui_macroquad::egui;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub ui_scale: f32,
    pub display_chunk_borders: bool,
    #[serde(skip, default = "default_true")]
    pub dirty: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            ui_scale: Self::DEFAULT_UI_SCALE,
            display_chunk_borders: false,
            dirty: true,
        }
    }
}

fn default_true() -> bool {
    true
}

impl Settings {
    pub const DEFAULT_UI_SCALE: f32 = 1.0;

    pub fn load() -> Self {
        let settings_string = Storage::get("settings");
        settings_string.map_or_else(Settings::default, |s| {
            serde_json::from_str(&s).unwrap_or_default()
        })
    }

    fn save(&self) {
        let Ok(settings_string) = serde_json::to_string(self) else {
            return;
        };
        Storage::set("settings", &settings_string);
    }

    pub fn apply(&mut self, ctx: &egui::Context) {
        if !self.dirty {
            return;
        }

        ctx.set_pixels_per_point(self.ui_scale);

        self.dirty = false;

        self.save();
    }
}
