use bevy::prelude::*;

use crate::{
    config::{Config, GameConfig},
    game::GameState,
};

#[derive(Resource)]
pub struct Lives(pub u32);

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PlayingBall), prepare_lifes);
    }
}

fn prepare_lifes(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    commands.insert_resource(Lives(config.stats.lifes));
}
