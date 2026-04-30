use crate::i18n::Translatable;
use egui_macroquad::egui::panel::TopBottomSide;
use lemon_colonies_core::lingo::Lingo::{Bottom, Top};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumIter;

pub mod action;
pub mod hover;
pub mod info;
pub mod meta;

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
