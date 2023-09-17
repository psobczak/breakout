use core::panic;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    ball::BallBouncedEvent,
    config::{BlockConfig, Config, GameConfig},
    game::{GameState, SpawningSet},
    paddle::Dimensions,
};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::PlayingBall),
            spawn_blocks.in_set(SpawningSet::Blocks),
        )
        .add_systems(Update, (hit_block,).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Block;

pub fn spawn_blocks(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let total_block_width = total_blocks_width(window, &config.block);

    let left_upper_corner = Vec2::new(
        total_block_width / 2.0,
        (window.height() / 2.0 - config.block.offset_from_top)
            - total_blocks_height(window, &config.block) / 2.0,
    );

    let Some(position) = camera.viewport_to_world_2d(camera_transform, left_upper_corner) else {
        panic!("could not calculate left upper block in world coordinates")
    };

    commands
        .spawn((
            Name::from("Blocks"),
            SpatialBundle {
                transform: Transform::from_xyz(0.0, left_upper_corner.y, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            for i in 1..config.block.columns {
                for j in 1..config.block.rows {
                    builder.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (position.x + config.block.width / 2.0)
                                    + (i as f32 * config.block.width)
                                    + (config.block.horizontal_offset * i as f32),
                                (position.y + config.block.height)
                                    - ((j as f32 * config.block.height)
                                        + (config.block.vertical_offset * (j as f32 + 1.0)))
                                    - config.block.offset_from_top,
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::GREEN,
                                custom_size: Some(Vec2::new(
                                    config.block.width,
                                    config.block.height,
                                )),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Dimensions(Vec2::new(config.block.width, config.block.height)),
                        Block,
                    ));
                }
            }
        });
}

fn total_blocks_width(window: &Window, block_config: &BlockConfig) -> f32 {
    window.width()
        - ((block_config.width * block_config.columns as f32 + 1.0)
            + block_config.columns as f32 * block_config.horizontal_offset)
}

fn total_blocks_height(window: &Window, block_config: &BlockConfig) -> f32 {
    window.height()
        - ((block_config.height * block_config.rows as f32 + 1.0)
            + (block_config.vertical_offset * block_config.rows as f32))
}

fn hit_block(
    mut commands: Commands,
    mut reader: EventReader<BallBouncedEvent>,
    blocks: Query<With<Block>>,
) {
    for event in reader.iter() {
        if blocks.get(event.0).is_ok() {
            commands.entity(event.0).despawn_recursive();
        }
    }
}
