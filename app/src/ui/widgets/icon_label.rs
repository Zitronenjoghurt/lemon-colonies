use crate::ui::icon::IconCache;
use egui_macroquad::egui;
use egui_macroquad::egui::{Response, Ui, Widget};
use lemon_colonies_core::game::icon::Icon;

pub struct IconLabel<'a> {
    icons: &'a IconCache,
    icon: Icon,
    label: String,
    color: Option<egui::Color32>,
    icon_size: f32,
    small: bool,
    tooltip: Option<String>,
}

impl<'a> IconLabel<'a> {
    pub fn new(icons: &'a IconCache, icon: Icon, label: impl Into<String>) -> Self {
        Self {
            icons,
            icon,
            label: label.into(),
            color: None,
            icon_size: 16.0,
            small: false,
            tooltip: None,
        }
    }

    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

impl Widget for IconLabel<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let layout =
            egui::Layout::left_to_right(egui::Align::Center).with_main_align(egui::Align::Center);

        let response = ui
            .with_layout(layout, |ui| {
                ui.add(self.icons.image(self.icon, self.icon_size));

                let mut text = egui::RichText::new(self.label);
                if self.small {
                    text = text.small();
                }
                if let Some(color) = self.color {
                    text = text.color(color);
                }

                ui.add(egui::Label::new(text).selectable(false));
            })
            .response;

        if let Some(tooltip_text) = self.tooltip
            && ui.rect_contains_pointer(response.rect)
        {
            egui::show_tooltip_at_pointer(ui.ctx(), response.layer_id, response.id, |ui| {
                ui.label(tooltip_text);
            });
        }

        response
    }
}
