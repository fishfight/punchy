use bevy::{
    math::{UVec2, Vec2, Vec3},
    prelude::{Color, Component, Handle, Image},
    reflect::TypeUuid,
    sprite::TextureAtlas,
    utils::HashMap,
};
use bevy_egui::egui;
use bevy_kira_audio::AudioSource;
use bevy_parallax::{LayerData, ParallaxResource};
use serde::Deserialize;

use crate::{animation::Clip, assets::EguiFont, state::State, Stats};

pub mod settings;
pub use settings::*;

pub use ui::*;
pub mod ui;

pub mod localization;
pub use localization::TranslationsMeta;

#[derive(TypeUuid, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[uuid = "eb28180f-ef68-44a0-8479-a299a3cef66e"]
pub struct GameMeta {
    pub start_level: String,
    #[serde(skip)]
    pub start_level_handle: Handle<LevelMeta>,
    pub main_menu: MainMenuMeta,
    pub ui_theme: UIThemeMeta,
    pub camera_height: u32,
    pub camera_move_right_boundary: f32,

    pub default_settings: Settings,
    pub translations: TranslationsMeta,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MainMenuMeta {
    pub title_font: FontMeta,
    pub background_image: ImageMeta,
    pub music: String,
    #[serde(skip)]
    pub music_handle: Handle<AudioSource>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageMeta {
    pub image: String,
    pub image_size: Vec2,
    #[serde(skip)]
    pub image_handle: Handle<Image>,
}

#[derive(TypeUuid, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[uuid = "32111f6e-bb9a-4ea7-8988-1220b923a059"]
pub struct LevelMeta {
    pub background_color: [u8; 3],
    pub parallax_background: ParallaxMeta,
    pub players: Vec<FighterSpawnMeta>,
    #[serde(default)]
    pub enemies: Vec<FighterSpawnMeta>,
    #[serde(default)]
    pub items: Vec<ItemSpawnMeta>,
    pub music: String,
    #[serde(skip)]
    pub music_handle: Handle<AudioSource>,
    pub stop_points: Vec<f32>,
}

impl LevelMeta {
    pub fn background_color(&self) -> Color {
        let [r, g, b] = self.background_color;
        Color::rgb_u8(r, g, b)
    }
}

#[derive(TypeUuid, Deserialize, Clone, Debug, Component)]
#[serde(deny_unknown_fields)]
#[uuid = "d5e040c4-3de7-4b8a-b6c2-27f82f58d8f0"]
pub struct FighterMeta {
    pub name: String,
    pub stats: Stats,
    pub hud: FighterHudMeta,
    pub spritesheet: FighterSpritesheetMeta,
    pub audio: AudioMeta,
}

#[derive(TypeUuid, Deserialize, Clone, Debug, Component)]
#[serde(deny_unknown_fields)]
#[uuid = "5e2db270-ec2e-013a-92a8-2cf05d71216b"]
pub struct ItemMeta {
    pub name: String,
    pub image: ImageMeta,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FighterHudMeta {
    pub portrait: ImageMeta,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FighterSpritesheetMeta {
    pub image: Vec<String>,
    #[serde(skip)]
    pub atlas_handle: Vec<Handle<TextureAtlas>>,
    pub tile_size: UVec2,
    pub columns: usize,
    pub rows: usize,
    pub animation_fps: f32,
    pub animations: HashMap<State, Clip>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AudioMeta {
    pub effects: HashMap<State, HashMap<usize, String>>,
    #[serde(skip)]
    pub effect_handles: HashMap<State, HashMap<usize, Handle<AudioSource>>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FighterSpawnMeta {
    pub fighter: String,
    #[serde(skip)]
    pub fighter_handle: Handle<FighterMeta>,
    pub location: Vec3,
}

#[derive(TypeUuid, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[uuid = "f5092550-ec30-013a-92a9-2cf05d71216b"]
pub struct ItemSpawnMeta {
    pub item: String,
    #[serde(skip)]
    pub item_handle: Handle<ItemMeta>,
    pub location: Vec3,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ParallaxMeta {
    pub layers: Vec<ParallaxLayerMeta>,
}

impl ParallaxMeta {
    pub fn get_resource(&self) -> ParallaxResource {
        ParallaxResource::new(self.layers.iter().cloned().map(Into::into).collect())
    }
}

// TODO: This struct is a workaround for the fact that `bevy_parallax::LayerData` isn't Clone.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ParallaxLayerMeta {
    pub speed: f32,
    pub path: String,
    pub tile_size: Vec2,
    pub cols: usize,
    pub rows: usize,
    pub scale: f32,
    pub z: f32,
    pub transition_factor: f32,
}

impl From<ParallaxLayerMeta> for LayerData {
    fn from(meta: ParallaxLayerMeta) -> Self {
        Self {
            speed: meta.speed,
            path: meta.path,
            tile_size: meta.tile_size,
            cols: meta.cols,
            rows: meta.rows,
            scale: meta.scale,
            z: meta.z,
            transition_factor: meta.transition_factor,
        }
    }
}
