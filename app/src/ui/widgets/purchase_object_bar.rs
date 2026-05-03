use crate::game::Game;
use crate::ui::icon::IconCache;
use crate::ui::widgets::purchase_object::PurchaseObjectWidget;
use egui_macroquad::egui;
use egui_macroquad::egui::{Response, ScrollArea, Widget};
use lemon_colonies_core::game::object::purchase::PurchasableObjectCategory;

pub struct PurchaseObjectBar<'a> {
    game: &'a mut Game,
    icons: &'a IconCache,
    category: PurchasableObjectCategory,
    icon_size: f32,
    item_width: f32,
}

impl<'a> PurchaseObjectBar<'a> {
    pub fn new(
        game: &'a mut Game,
        icons: &'a IconCache,
        category: PurchasableObjectCategory,
    ) -> Self {
        Self {
            game,
            icons,
            category,
            icon_size: 32.0,
            item_width: 100.0,
        }
    }

    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    pub fn item_width(mut self, width: f32) -> Self {
        self.item_width = width;
        self
    }
}

impl Widget for PurchaseObjectBar<'_> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.style_mut().interaction.selectable_labels = false;
        ScrollArea::horizontal()
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for object in self.category.objects() {
                        PurchaseObjectWidget::new(self.game, self.icons, *object)
                            .icon_size(self.icon_size)
                            .width(self.item_width)
                            .ui(ui);
                    }
                })
                .response
            })
            .inner
    }
}
