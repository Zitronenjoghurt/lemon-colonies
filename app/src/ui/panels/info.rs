use crate::i18n::TranslatablePlural;
use crate::ui::{phosphor, UiViewer};
use egui_macroquad::egui;
use egui_macroquad::egui::{Grid, ScrollArea, SidePanel};
use lemon_colonies_core::game::resource::ResourceId;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, EnumIter, serde::Serialize, serde::Deserialize,
)]
pub enum InfoPanelTab {
    #[default]
    Inventory,
}

impl InfoPanelTab {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Inventory => phosphor::TREASURE_CHEST,
        }
    }
}

pub fn show(ctx: &egui::Context, v: &mut UiViewer) {
    SidePanel::left("info_panel")
        .default_width(10.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                InfoPanelTab::iter().for_each(|tab| {
                    ui.selectable_value(&mut v.state.info_panel_tab, tab, tab.icon());
                })
            });

            ui.separator();

            match v.state.info_panel_tab {
                InfoPanelTab::Inventory => show_inventory(ui, v),
            }
        });
}

fn show_inventory(ui: &mut egui::Ui, v: &mut UiViewer) {
    let Some(resources) = v.game.data.resources.value() else {
        return;
    };
    ScrollArea::vertical().show(ui, |ui| {
        Grid::new("info_panel_inventory_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                for rid in ResourceId::iter() {
                    ui.horizontal(|ui| {
                        ui.add(v.icons.image(rid.icon(), 16.0));
                        ui.small(rid.tp());
                    });
                    ui.small(resources.get(rid).to_string());
                    ui.end_row();
                }
            });
    });
}
