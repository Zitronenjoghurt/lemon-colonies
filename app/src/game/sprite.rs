use crate::game::atlas::{Atlas, AtlasStore};
use egui_macroquad::macroquad::prelude::{
    draw_texture_ex, vec2, Color, DrawTextureParams, Rect, Vec2, WHITE,
};
use lemon_colonies_core::game::object::ObjectData;
use lemon_colonies_core::game::terrain::Terrain;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    pub atlas: Atlas,
    pub src: Rect,
    pub pivot: Vec2,
    pub y_sort_offset: f32,
}

impl Sprite {
    pub const fn new(atlas: Atlas, src: Rect) -> Self {
        Self {
            atlas,
            src,
            pivot: vec2(src.w / 2.0, src.h),
            y_sort_offset: 0.0,
        }
    }

    pub const fn from_grid(atlas: Atlas, x: u32, y: u32) -> Self {
        Self {
            atlas,
            src: Rect::new(x as f32 * 8.0, y as f32 * 8.0, 8.0, 8.0),
            pivot: vec2(8.0 / 2.0, 8.0),
            y_sort_offset: 0.0,
        }
    }

    pub const fn from_object(atlas: Atlas, x: f32, y: f32, object: &ObjectData) -> Self {
        Self {
            atlas,
            src: Rect::new(x, y, object.width(), object.height()),
            pivot: vec2(object.pivot().0, object.pivot().1),
            y_sort_offset: 0.0,
        }
    }

    pub fn width(&self) -> f32 {
        self.src.w
    }

    pub fn height(&self) -> f32 {
        self.src.h
    }
}

pub struct SpriteDraw {
    pub sprite: Sprite,
    pub anchor: Vec2,
    pub sort_y: f32,
    pub tint: Color,
}

impl SpriteDraw {
    pub fn new(sprite: Sprite, anchor: Vec2) -> Self {
        let sort_y = anchor.y + sprite.y_sort_offset;
        Self {
            sprite,
            anchor,
            sort_y,
            tint: WHITE,
        }
    }

    pub fn with_tint(mut self, tint: Color) -> Self {
        self.tint = tint;
        self
    }

    pub fn draw(&self, store: &AtlasStore) {
        let pivot = self.sprite.pivot;
        draw_texture_ex(
            store.get(self.sprite.atlas),
            self.anchor.x - pivot.x,
            self.anchor.y - pivot.y,
            self.tint,
            DrawTextureParams {
                source: Some(self.sprite.src),
                dest_size: Some(vec2(self.sprite.width(), self.sprite.height())),
                ..Default::default()
            },
        );
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

impl HasSprite for ObjectData {
    fn sprite(&self) -> Sprite {
        match self {
            Self::Bush => Sprite::from_object(Atlas::BaseOverworld, 3.0, 654.0, self),
        }
    }
}
