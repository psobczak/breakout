use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    config::{Config, GameConfig},
    game::GameState,
    paddle::Dimensions,
};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (spawn_blocks,));
    }
}

#[derive(Component)]
pub struct Block;

pub fn spawn_blocks(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    commands
        .spawn((Name::from("Blocks"), SpatialBundle::default()))
        .with_children(|builder| {
            builder.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(config.block.width, config.block.height)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Dimensions(Vec2::new(config.block.width, config.block.height)),
                Block,
            ));
        });
}
