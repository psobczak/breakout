use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use crate::{
    config::{Config, GameConfig},
    game::GameState,
    paddle::Speed,
};

#[derive(Component)]
struct Ball {
    radius: f32,
}

#[derive(Event)]
enum BallTouchedEdge {
    Left(Entity),
    Right(Entity),
    Top(Entity),
}

#[derive(Resource, Debug, Default)]
struct Bounces(u32);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Bounces::default())
            .add_event::<BallTouchedEdge>()
            .add_systems(OnEnter(GameState::Game), spawn_ball)
            .add_systems(
                Update,
                (detect_edge, change_ball_direction, move_ball)
                    .chain()
                    .distributive_run_if(in_state(GameState::Game)),
            );
    }
}

fn detect_edge(
    ball: Query<(&Ball, &GlobalTransform, Entity)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut writer: EventWriter<BallTouchedEdge>,
) {
    let window = window.single();

    for (ball, transform, entity) in &ball {
        if transform.translation().y >= (window.height() / 2.0) - ball.radius {
            writer.send(BallTouchedEdge::Top(entity));
        }

        if transform.translation().x >= (window.width() / 2.0) - ball.radius {
            writer.send(BallTouchedEdge::Right(entity));
        }

        if transform.translation().x <= (-window.width() / 2.0) + ball.radius {
            writer.send(BallTouchedEdge::Left(entity));
        }
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    game_config: Res<GameConfig>,
    assets: Res<Assets<Config>>,
) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(config.ball.radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(config.ball.color)),
            ..default()
        },
        Ball {
            radius: config.ball.radius,
        },
        Speed(Vec2::new(config.ball.initial_speed, 150.0)),
    ));
}

fn move_ball(mut ball: Query<(&mut Transform, &Speed), With<Ball>>, time: Res<Time>) {
    for (mut transform, speed) in &mut ball {
        transform.translation.y += speed.0.y * time.delta_seconds();
        transform.translation.x -= speed.0.x * time.delta_seconds();
    }
}

fn change_ball_direction(
    mut balls: Query<&mut Speed, With<Ball>>,
    mut reader: EventReader<BallTouchedEdge>,
) {
    for event in reader.iter() {
        match event {
            BallTouchedEdge::Left(entity) => {
                if let Ok(mut speed) = balls.get_mut(*entity) {
                    speed.0.x *= -1.0;
                }
            }
            BallTouchedEdge::Right(entity) => {
                if let Ok(mut speed) = balls.get_mut(*entity) {
                    speed.0.x *= -1.0;
                }
            }
            BallTouchedEdge::Top(entity) => {
                if let Ok(mut speed) = balls.get_mut(*entity) {
                    speed.0.y *= -1.0;
                }
            }
        };
    }
}
