use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
};

use crate::{
    config::{Config, GameConfig},
    debug::MousePosition,
    game::{AppState, BoundingBox, PlayState, SpawningSet},
    paddle::{Dimensions, Paddle, Speed},
};

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
}

#[derive(Resource, Debug, Default)]
pub struct Bounces(pub u32);

#[derive(Event, Debug)]
pub struct BallCollisionEvent {
    pub ball: Entity,
    pub with: Entity,
    pub collision: Collision,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BallCollisionEvent>()
            .insert_resource(Bounces::default())
            .add_systems(
                OnEnter(AppState::Playing),
                spawn_ball.in_set(SpawningSet::Ball),
            )
            .add_systems(
                Update,
                (follow_paddle, play_ball, calculate_ball_direction).distributive_run_if(
                    in_state(AppState::Playing).and_then(in_state(PlayState::ReadyToShoot)),
                ),
            )
            .add_systems(
                Update,
                (
                    (detect_collision, change_ball_direction, move_ball).chain(),
                    increase_ball_speed.run_if(resource_changed::<Bounces>()),
                    ball_touched_bottom,
                )
                    .distributive_run_if(
                        in_state(AppState::Playing).and_then(in_state(PlayState::BallInGame)),
                    ),
            )
            .add_systems(
                OnExit(PlayState::BallInGame),
                (follow_paddle, reset_speed_on_new_life),
            );
    }
}

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

fn reset_speed_on_new_life(
    mut balls: Query<&mut Speed, With<Ball>>,
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    for mut speed in &mut balls {
        speed.0 = Vec2::new(config.ball.initial_speed, 150.0);
    }
}

fn change_ball_direction(
    mut balls: Query<&mut Speed, With<Ball>>,
    mut reader: EventReader<BallCollisionEvent>,
    mut bounces: ResMut<Bounces>,
) {
    for event in reader.iter() {
        if let Ok(mut speed) = balls.get_mut(event.ball) {
            match event.collision {
                Collision::Left | Collision::Right => {
                    speed.0.x *= -1.0;
                    bounces.0 += 1;
                }
                Collision::Top | Collision::Bottom => {
                    speed.0.y *= -1.0;
                    bounces.0 += 1;
                }
                Collision::Inside => {}
            }
        }
    }
}

fn detect_collision(
    balls: Query<(&Ball, &GlobalTransform, Entity), With<Ball>>,
    bouncable: Query<(&GlobalTransform, &Dimensions, Entity)>,
    mut writer: EventWriter<BallCollisionEvent>,
) {
    for (paddle_transform, dimensions, bouncable_entity) in &bouncable {
        for (ball, ball_transform, ball_entity) in &balls {
            if let Some(collision) = collide(
                ball_transform.translation(),
                Vec2::splat(ball.radius),
                paddle_transform.translation(),
                dimensions.0,
            ) {
                match collision {
                    Collision::Left => writer.send(BallCollisionEvent {
                        ball: ball_entity,
                        with: bouncable_entity,
                        collision: Collision::Left,
                    }),
                    Collision::Right => writer.send(BallCollisionEvent {
                        ball: ball_entity,
                        with: bouncable_entity,
                        collision: Collision::Right,
                    }),
                    Collision::Top => writer.send(BallCollisionEvent {
                        ball: ball_entity,
                        with: bouncable_entity,
                        collision: Collision::Top,
                    }),
                    Collision::Bottom => writer.send(BallCollisionEvent {
                        ball: ball_entity,
                        with: bouncable_entity,
                        collision: Collision::Bottom,
                    }),
                    Collision::Inside => {}
                }
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
        transform.translation.y += speed.0.y * time.delta_seconds() * 150.0;
        transform.translation.x -= speed.0.x * time.delta_seconds() * 150.0;
    }
}

fn follow_paddle(
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
    paddle: Query<&GlobalTransform, With<Paddle>>,
    mut ball: Query<&mut Transform, With<Ball>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    let paddle = paddle.single();
    let mut ball = ball.single_mut();

    ball.translation.x = paddle.translation().x;
    ball.translation.y = paddle.translation().y + config.ball.offset_from_paddle;
}

fn play_ball(input: Res<Input<MouseButton>>, mut state: ResMut<NextState<PlayState>>) {
    if input.just_pressed(MouseButton::Left) {
        state.set(PlayState::BallInGame)
    }
}

fn ball_touched_bottom(
    mut reader: EventReader<BallCollisionEvent>,
    bounding_box: Query<With<BoundingBox>>,
    mut state: ResMut<NextState<PlayState>>,
) {
    for event in reader.iter() {
        if bounding_box.get(event.with).is_ok() && event.collision == Collision::Bottom {
            state.set(PlayState::ReadyToShoot)
        }
    }
}

fn calculate_ball_direction(
    paddle: Query<&GlobalTransform, With<Paddle>>,
    mouse_position: Res<MousePosition>,
    mut ball: Query<&mut Speed, With<Ball>>,
) {
    let paddle = paddle.single();
    let paddle_translation = paddle.translation();
    let mut ball = ball.single_mut();

    let mut to_cursor = (mouse_position.world - paddle_translation.truncate()).normalize();
    to_cursor.x *= -1.0;

    ball.0 = to_cursor;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_change_state_on_lmb() {
        let mut app = App::new();
        app.add_state::<PlayState>();

        let mut input = Input::<MouseButton>::default();
        input.press(MouseButton::Left);
        app.insert_resource(input);

        app.add_systems(
            Update,
            (play_ball, apply_state_transition::<PlayState>).chain(),
        );

        app.update();

        let state = app.world.resource::<State<PlayState>>().get();
        assert_eq!(*state, PlayState::BallInGame);
    }

    #[test]
    fn should_change_play_state_on_ball_touching_bottom() {
        let mut app = App::new();
        app.add_event::<BallCollisionEvent>();
        app.add_state::<PlayState>();

        app.world
            .resource_mut::<NextState<PlayState>>()
            .set(PlayState::BallInGame);

        let bounding_box = app.world.spawn(BoundingBox).id();
        let ball = app.world.spawn(Ball { radius: 10.0 }).id();

        app.world
            .resource_mut::<Events<BallCollisionEvent>>()
            .send(BallCollisionEvent {
                ball,
                with: bounding_box,
                collision: Collision::Bottom,
            });

        app.add_systems(
            Update,
            (
                apply_state_transition::<PlayState>,
                ball_touched_bottom,
                apply_state_transition::<PlayState>,
            )
                .chain(),
        );

        app.update();

        let state = app.world.resource::<State<PlayState>>().get();
        assert_eq!(*state, PlayState::ReadyToShoot);
    }
}
