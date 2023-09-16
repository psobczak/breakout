use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
};

use crate::{
    config::{Config, GameConfig},
    game::{GameState, SpawningSet},
    paddle::{Dimensions, Paddle, Speed},
};

#[derive(Component)]
pub struct Ball {
    radius: f32,
}

#[derive(Resource, Debug, Default)]
pub struct Bounces(pub u32);
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BallBouncedEvent>()
            .insert_resource(Bounces::default())
            .add_systems(
                OnEnter(GameState::PlayingBall),
                spawn_ball.in_set(SpawningSet::Ball),
            )
            .add_systems(
                Update,
                (follow_paddle).run_if(in_state(GameState::PlayingBall)),
            )
            .add_systems(
                Update,
                (
                    (bounce_ball, move_ball).chain(),
                    increase_ball_speed.run_if(resource_changed::<Bounces>()),
                )
                    .distributive_run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Event)]
pub struct BallBouncedEvent(pub Entity);

fn increase_ball_speed(
    mut balls: Query<&mut Speed, With<Ball>>,
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    for mut speed in &mut balls {
        speed.0.x += speed.0.x * (config.ball.speed_increase / 100.0);
        speed.0.y += speed.0.y * (config.ball.speed_increase / 100.0);
    }
}

fn bounce_ball(
    mut balls: Query<(&mut Speed, &Ball, &GlobalTransform), With<Ball>>,
    bouncable: Query<(&GlobalTransform, &Dimensions, Entity)>,
    mut bounces: ResMut<Bounces>,
    mut writer: EventWriter<BallBouncedEvent>,
) {
    for (paddle_transform, dimensions, entity) in &bouncable {
        for (mut speed, ball, ball_transform) in &mut balls {
            if let Some(collision) = collide(
                ball_transform.translation(),
                Vec2::splat(ball.radius),
                paddle_transform.translation(),
                dimensions.0,
            ) {
                match collision {
                    Collision::Left | Collision::Right => {
                        speed.0.x *= -1.0;
                        bounces.0 += 1;
                        writer.send(BallBouncedEvent(entity));
                    }
                    Collision::Top | Collision::Bottom => {
                        speed.0.y *= -1.0;
                        bounces.0 += 1;
                        writer.send(BallBouncedEvent(entity));
                    }
                    Collision::Inside => {}
                };
            }
        }
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    paddle: Query<&Transform, With<Paddle>>,
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    let paddle_transform = paddle.single();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(config.ball.radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(config.ball.color)),
            transform: Transform::from_xyz(
                paddle_transform.translation.x,
                paddle_transform.translation.y + config.ball.offset_from_paddle,
                0.0,
            ),
            ..Default::default()
        },
        Ball {
            radius: config.ball.radius,
        },
        Speed(Vec2::new(config.ball.initial_speed, 150.0)),
        Name::from("Ball"),
    ));
}

fn move_ball(mut ball: Query<(&mut Transform, &Speed), With<Ball>>, time: Res<Time>) {
    for (mut transform, speed) in &mut ball {
        // transform.translation.y += speed.0.y * time.delta_seconds();
        // transform.translation.x -= speed.0.x * time.delta_seconds();
    }
}

fn follow_paddle(
    paddle: Query<&GlobalTransform, With<Paddle>>,
    mut ball: Query<&mut Transform, With<Ball>>,
) {
    let paddle = paddle.single();
    let mut ball = ball.single_mut();

    ball.translation.x = paddle.translation().x;
}
