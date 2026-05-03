use crate::game::Game;
use crate::i18n::Translatable;
use crate::ui::icon::IconCache;
use crate::ui::widgets::icon_label::IconLabel;
use egui_macroquad::egui;
use egui_macroquad::egui::{Response, Ui, Widget};
use lemon_colonies_core::game::object::purchase::PurchasableObject;

pub struct PurchaseObjectWidget<'a> {
    game: &'a mut Game,
    icons: &'a IconCache,
    object: PurchasableObject,
    icon_size: f32,
    width: f32,
}

impl<'a> PurchaseObjectWidget<'a> {
    pub fn new(game: &'a mut Game, icons: &'a IconCache, object: PurchasableObject) -> Self {
        Self {
            game,
            icons,
            object,
            icon_size: 32.0,
            width: 100.0,
        }
    }

    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Widget for PurchaseObjectWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let resources = self.game.data.resources.value();

        let enabled = if let Some(resources) = resources {
            self.object.can_buy(resources)
        } else {
            false
        };

        let sense = if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::empty()
        };

        let response = ui
            .add_enabled_ui(enabled, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.set_max_width(self.width);
                    ui.set_min_width(self.width);
                    ui.vertical_centered(|ui| {
                        ui.add(self.icons.image(self.object.icon(), self.icon_size));
                        ui.label(egui::RichText::new(self.object.t().to_string()).strong());

                        for (resource, amt) in self.object.base_costs() {
                            let has_resource = resources.is_some_and(|r| r.get(*resource) >= *amt);

                            let color = if has_resource {
                                ui.visuals().text_color()
                            } else {
                                ui.visuals().error_fg_color
                            };

                            IconLabel::new(self.icons, resource.icon(), amt.to_string())
                                .color(color)
                                .tooltip(resource.t())
                                .ui(ui);
                        }
                    });
                })
            })
            .inner
            .response;

        let response = response.interact(sense);

        if response.hovered()
            || self
                .game
                .object_place
                .purchasable_object()
                .is_some_and(|o| o == self.object)
        {
            ui.painter()
                .rect_filled(response.rect, 4.0, egui::Color32::from_white_alpha(5));
        }

        if response.is_pointer_button_down_on() {
            ui.painter()
                .rect_filled(response.rect, 4.0, egui::Color32::from_black_alpha(10));
        }

        if response.clicked() {
            self.game.object_place.purchase(self.object);
        }

        response
    }
}
