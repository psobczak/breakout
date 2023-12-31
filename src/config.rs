use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;

use crate::game::AppState;

pub struct ConfigPlugin;

#[derive(AssetCollection, Resource, Debug, TypeUuid, TypePath)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct GameConfig {
    #[asset(path = "game.config.yaml")]
    pub config: Handle<Config>,
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(YamlAssetPlugin::<Config>::new(&["config.yaml"]))
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::Menu),
            )
            .add_collection_to_loading_state::<_, GameConfig>(AppState::AssetLoading);
    }
}

#[derive(serde::Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Config {
    pub ball: BallConfig,
    pub paddle: PaddleConfig,
    pub block: BlockConfig,
    pub stats: StatsConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct BallConfig {
    pub radius: f32,
    pub color: Color,
    pub initial_speed: f32,
    pub speed_increase: f32,
    pub offset_from_paddle: f32,
}

#[derive(serde::Deserialize, Debug)]
pub struct PaddleConfig {
    pub width: f32,
    pub height: f32,
    pub offset_from_bottom: f32,
    pub color: Color,
    pub initial_speed: f32,
}

#[derive(serde::Deserialize, Debug)]
pub struct BlockConfig {
    pub width: f32,
    pub height: f32,
    pub horizontal_offset: f32,
    pub vertical_offset: f32,
    pub rows: u32,
    pub columns: u32,
    pub offset_from_top: f32,
}

#[derive(serde::Deserialize, Debug)]
pub struct StatsConfig {
    pub lifes: u32,
}
