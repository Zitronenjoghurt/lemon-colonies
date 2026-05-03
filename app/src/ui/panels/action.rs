use crate::i18n::Translatable;
use crate::ui::panels::sub_action::SubActionTab;
use crate::ui::UiViewer;
use egui_macroquad::egui;
use egui_macroquad::egui::panel::TopBottomSide;
use egui_macroquad::egui::TopBottomPanel;
use lemon_colonies_core::lingo::Lingo::{Bottom, Top};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn show(ctx: &egui::Context, v: &mut UiViewer) {
    let frame = egui::Frame::side_top_panel(&ctx.style()).inner_margin(8.0);
    TopBottomPanel::new(v.settings.action_panel_pos.egui(), "action_panel")
        .frame(frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                for tab in SubActionTab::iter() {
                    let selected = v.state.sub_action_panel_tab == Some(tab);
                    let response =
                        ui.selectable_label(selected, format!("{} {}", tab.icon(), tab.label()));
                    if response.clicked() {
                        v.state.sub_action_panel_tab = if selected { None } else { Some(tab) };
                    }
                }
            });
        });
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum ActionPanelPosition {
    #[default]
    Bottom,
    Top,
}

impl ActionPanelPosition {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Bottom => Self::Top,
            Self::Top => Self::Bottom,
        }
    }

    pub fn egui(&self) -> TopBottomSide {
        match self {
            Self::Bottom => TopBottomSide::Bottom,
            Self::Top => TopBottomSide::Top,
        }
    }
}

impl Display for ActionPanelPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bottom => write!(f, "{}", Bottom.t()),
            Self::Top => write!(f, "{}", Top.t()),
        }
    }
}
