use crate::game::atlas::Atlas;
use egui_macroquad::macroquad::prelude::Rect;
use lemon_colonies_core::game::terrain::Terrain;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    pub atlas: Atlas,
    pub src: Rect,
}

impl Sprite {
    pub const fn from_grid(atlas: Atlas, x: u32, y: u32) -> Self {
        Self {
            atlas,
            src: Rect::new(x as f32 * 8.0, y as f32 * 8.0, 8.0, 8.0),
        }
    }
}

pub trait HasSprite {
    fn sprite(&self) -> Sprite;
}

impl HasSprite for Terrain {
    fn sprite(&self) -> Sprite {
        match self {
            Self::GrassPlain => Sprite::from_grid(Atlas::BaseOverworld, 0, 0),
            Self::GrassVariant1 => Sprite::from_grid(Atlas::BaseOverworld, 1, 0),
            Self::GrassVariant2 => Sprite::from_grid(Atlas::BaseOverworld, 2, 0),
            Self::GrassVariant3 => Sprite::from_grid(Atlas::BaseOverworld, 3, 0),
            Self::GrassVariant4 => Sprite::from_grid(Atlas::BaseOverworld, 0, 1),
            Self::GrassVariant5 => Sprite::from_grid(Atlas::BaseOverworld, 1, 1),
            Self::GrassVariant6 => Sprite::from_grid(Atlas::BaseOverworld, 2, 1),
            Self::GrassVariant7 => Sprite::from_grid(Atlas::BaseOverworld, 3, 1),
            Self::GrassFlowersRoundYellowBig => Sprite::from_grid(Atlas::BaseOverworld, 0, 2),
            Self::GrassFlowersRoundYellowSmall => Sprite::from_grid(Atlas::BaseOverworld, 1, 2),
            Self::GrassFlowersRoundCyanBig => Sprite::from_grid(Atlas::BaseOverworld, 2, 2),
            Self::GrassFlowersRoundCyanSmall => Sprite::from_grid(Atlas::BaseOverworld, 3, 2),
            Self::GrassFlowersRoundMagentaBig => Sprite::from_grid(Atlas::BaseOverworld, 0, 3),
            Self::GrassFlowersRoundMagentaSmall => Sprite::from_grid(Atlas::BaseOverworld, 1, 3),
            Self::GrassFlowersRoundWhiteBig => Sprite::from_grid(Atlas::BaseOverworld, 2, 3),
            Self::GrassFlowersRoundWhiteSmall => Sprite::from_grid(Atlas::BaseOverworld, 3, 3),
            Self::GrassFlowersCrossWhiteBig => Sprite::from_grid(Atlas::BaseOverworld, 0, 4),
            Self::GrassFlowersCrossWhiteSmall => Sprite::from_grid(Atlas::BaseOverworld, 1, 4),
            Self::GrassFlowersCrossYellowBig => Sprite::from_grid(Atlas::BaseOverworld, 2, 4),
            Self::GrassFlowersCrossYellowSmall => Sprite::from_grid(Atlas::BaseOverworld, 3, 4),
            Self::GrassFlowersCrossCyanBig => Sprite::from_grid(Atlas::BaseOverworld, 0, 5),
            Self::GrassFlowersCrossCyanSmall => Sprite::from_grid(Atlas::BaseOverworld, 1, 5),
            Self::GrassFlowersCrossMagentaBig => Sprite::from_grid(Atlas::BaseOverworld, 2, 5),
            Self::GrassFlowersCrossMagentaSmall => Sprite::from_grid(Atlas::BaseOverworld, 3, 5),
            Self::GrassShroomsRedBig => Sprite::from_grid(Atlas::BaseOverworld, 0, 6),
            Self::GrassShroomsBrownBig => Sprite::from_grid(Atlas::BaseOverworld, 1, 6),
            Self::GrassShroomsRedSmall => Sprite::from_grid(Atlas::BaseOverworld, 2, 6),
            Self::GrassShroomsBrownSmall => Sprite::from_grid(Atlas::BaseOverworld, 3, 6),
        }
    }
}
