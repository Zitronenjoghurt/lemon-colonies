use crate::i18n::Translatable;
use crate::ui::widgets::purchase_object_bar::PurchaseObjectBar;
use crate::ui::{phosphor, UiViewer};
use egui_macroquad::egui;
use egui_macroquad::egui::{TopBottomPanel, Widget};
use lemon_colonies_core::game::object::purchase::PurchasableObjectCategory;
use lemon_colonies_core::lingo::Lingo::Plants;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

pub fn show(ctx: &egui::Context, v: &mut UiViewer) {
    let frame = egui::Frame::side_top_panel(&ctx.style()).inner_margin(8.0);
    TopBottomPanel::new(v.settings.action_panel_pos.egui(), "sub_action_panel")
        .frame(frame)
        .show_animated(ctx, v.state.sub_action_panel_tab.is_some(), |ui| {
            if let Some(tab) = v.state.sub_action_panel_tab {
                tab.ui(ui, v);
            }
        });
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum SubActionTab {
    Plants,
}

impl SubActionTab {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Plants => phosphor::PLANT,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Plants => Plants.t().to_string(),
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui, v: &mut UiViewer) {
        match self {
            Self::Plants => {
                PurchaseObjectBar::new(v.game, v.icons, PurchasableObjectCategory::Plants).ui(ui);
            }
        }
    }
}
