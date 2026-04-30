use crate::game::atlas::Atlas;
use crate::game::sprite::{HasSprite, Sprite};
use egui_macroquad::egui;
use lemon_colonies_core::game::icon::Icon;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub struct IconCache {
    textures: HashMap<Icon, egui::TextureHandle>,
}

impl IconCache {
    pub fn load(ctx: &egui::Context) -> Self {
        let atlas_images: HashMap<Atlas, image::RgbaImage> = Atlas::iter()
            .map(|a| {
                let img = image::load_from_memory(a.data())
                    .expect("failed to decode atlas")
                    .to_rgba8();
                (a, img)
            })
            .collect();

        let textures = Icon::iter()
            .map(|icon| {
                let sprite = icon.sprite();
                let atlas_img = &atlas_images[&sprite.atlas];

                let (x, y, w, h) = (
                    sprite.src.x as u32,
                    sprite.src.y as u32,
                    sprite.src.w as u32,
                    sprite.src.h as u32,
                );

                let cropped = image::imageops::crop_imm(atlas_img, x, y, w, h).to_image();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [w as usize, h as usize],
                    cropped.as_raw(),
                );

                let handle = ctx.load_texture(
                    format!("icon_{icon:?}"),
                    color_image,
                    egui::TextureOptions::NEAREST,
                );

                (icon, handle)
            })
            .collect();

        Self { textures }
    }

    pub fn image(&self, icon: Icon, size: f32) -> egui::Image<'_> {
        let handle = &self.textures[&icon];
        egui::Image::new(handle).fit_to_exact_size(egui::vec2(size, size))
    }
}

impl HasSprite for Icon {
    fn sprite(&self) -> Sprite {
        match self {
            Self::BlueberryBush => Sprite::from_icon(Atlas::BaseOverworld, 51, 654, 10, 10),
            Self::RaspberryBush => Sprite::from_icon(Atlas::BaseOverworld, 99, 654, 10, 10),
            Self::GolberryBush => Sprite::from_icon(Atlas::BaseOverworld, 147, 654, 10, 10),
            Self::Blueberries => Sprite::from_icon(Atlas::BaseItems, 16, 208, 8, 8),
            Self::Raspberries => Sprite::from_icon(Atlas::BaseItems, 32, 208, 8, 8),
            Self::Golberries => Sprite::from_icon(Atlas::BaseItems, 48, 208, 8, 8),
        }
    }
}
