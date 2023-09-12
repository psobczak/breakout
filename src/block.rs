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

    let block_config = &config.block;

    commands
        .spawn((Name::from("Blocks"), SpatialBundle::default()))
        .with_children(|builder| {
            for i in 0..10 {
                for j in 0..10 {
                    builder.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (i as f32 * block_config.width)
                                    + (block_config.horizontal_offset * i as f32),
                                (j as f32 * block_config.height)
                                    + (block_config.vertical_offset * j as f32),
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::GREEN,
                                custom_size: Some(Vec2::new(
                                    block_config.width,
                                    block_config.height,
                                )),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Dimensions(Vec2::new(block_config.width, block_config.height)),
                        Block,
                    ));
                }
            }
        });
}
