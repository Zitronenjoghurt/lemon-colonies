use crate::settings::Settings;
use crate::ui::widgets::reset_slider::ResetSlider;
use egui_macroquad::egui::{Grid, Response, ScrollArea, Ui, Widget};

pub struct SettingsWidget<'a> {
    pub settings: &'a mut Settings,
}

impl<'a> SettingsWidget<'a> {
    pub fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }

    fn content(&mut self, ui: &mut Ui) {
        self.ui_scale(ui);
    }

    fn ui_scale(&mut self, ui: &mut Ui) {
        ui.label("UI Scale");
        let response = ResetSlider::new(&mut self.settings.ui_scale, 0.5..=4.0)
            .step_by(0.1)
            .default_value(Settings::DEFAULT_UI_SCALE)
            .ui(ui);
        if response.drag_stopped() || (response.changed() && !response.dragged()) {
            self.settings.dirty = true;
        }
        ui.end_row();
    }
}

impl Widget for SettingsWidget<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ScrollArea::vertical()
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    Grid::new("settings_grid").num_columns(2).show(ui, |ui| {
                        self.content(ui);
                    })
                })
                .response
            })
            .inner
    }
}
