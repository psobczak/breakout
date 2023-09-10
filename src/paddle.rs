use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_debug_lines::*;

use crate::{
    config::{Config, GameConfig},
    game::GameState,
};

#[derive(Component, Deref, DerefMut, Debug, Reflect)]
pub struct Speed(pub Vec2);

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Reflect)]
pub struct Dimensions(pub Vec2);

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .register_type::<Dimensions>()
            .add_systems(OnEnter(GameState::Game), spawn_paddle)
            .add_systems(
                Update,
                (move_paddle, draw_debug_lines).distributive_run_if(in_state(GameState::Game)),
            );
    }
}

fn spawn_paddle(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    let window = window.single();

    commands.spawn((
        Speed(Vec2::new(config.paddle.initial_speed, 0.0)),
        Paddle,
        Dimensions(Vec2::new(config.paddle.width, config.paddle.height)),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(config.paddle.width, config.paddle.height)),
                color: config.paddle.color,
                ..Default::default()
            },
            transform: Transform::from_xyz(
                0.0,
                (-window.height() / 2.0) + config.paddle.offset_from_bottom,
                0.0,
            ),
            ..Default::default()
        },
    ));
}

fn move_paddle(
    mut paddle: Query<(&mut Transform, &Speed), With<Paddle>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, speed) = paddle.single_mut();

    if input.any_pressed([KeyCode::Left, KeyCode::A]) {
        transform.translation.x -= time.delta_seconds() * speed.0.x
    }

    if input.any_pressed([KeyCode::Right, KeyCode::D]) {
        transform.translation.x += time.delta_seconds() * speed.0.x
    }
}

fn draw_debug_lines(
    mut lines: ResMut<DebugLines>,
    paddle: Query<&GlobalTransform, With<Paddle>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let paddle_transform = paddle.single();
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    if let Some(position) = window.cursor_position() {
        let start = paddle_transform.translation();
        let duration = 0.0;

        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, position) {
            lines.line(start, world_position.extend(0.0), duration);
        }
    }
}
