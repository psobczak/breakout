use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    config::{Config, GameConfig},
    game::GameState,
};

#[derive(Component, Deref, DerefMut, Debug, Reflect)]
pub struct Speed(pub Vec2);

#[derive(Component)]
pub struct Paddle {
    pub size: Vec2,
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .add_systems(OnEnter(GameState::Game), spawn_paddle)
            .add_systems(
                Update,
                (move_paddle,).distributive_run_if(in_state(GameState::Game)),
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
        Paddle {
            size: Vec2::new(config.paddle.width, config.paddle.height),
        },
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