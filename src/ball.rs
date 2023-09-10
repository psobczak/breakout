use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};

use crate::{
    config::{Config, GameConfig},
    game::GameState,
    paddle::{Dimensions, Speed},
};

#[derive(Component)]
struct Ball {
    radius: f32,
}

#[derive(Resource, Debug, Default)]
pub struct Bounces(pub u32);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Bounces::default())
            .add_systems(OnEnter(GameState::Game), spawn_ball)
            .add_systems(
                Update,
                ((bounce_ball, move_ball).chain(),).distributive_run_if(in_state(GameState::Game)),
            );
    }
}

fn bounce_ball(
    mut balls: Query<(&mut Speed, &Ball, &GlobalTransform), With<Ball>>,
    bouncable: Query<(&GlobalTransform, &Dimensions)>,
    mut bounces: ResMut<Bounces>,
) {
    for (paddle_transform, dimensions) in &bouncable {
        for (mut speed, ball, ball_transform) in &mut balls {
            if let Some(collision) = collide(
                ball_transform.translation(),
                Vec2::splat(ball.radius),
                paddle_transform.translation(),
                dimensions.0,
            ) {
                match collision {
                    bevy::sprite::collide_aabb::Collision::Left => {
                        speed.0.x *= -1.0;
                        bounces.0 += 1;
                    }
                    bevy::sprite::collide_aabb::Collision::Right => {
                        speed.0.x *= -1.0;
                        bounces.0 += 1;
                    }
                    bevy::sprite::collide_aabb::Collision::Top => {
                        speed.0.y *= -1.0;
                        bounces.0 += 1;
                    }
                    bevy::sprite::collide_aabb::Collision::Bottom => {
                        speed.0.y *= -1.0;
                        bounces.0 += 1;
                    }
                    bevy::sprite::collide_aabb::Collision::Inside => {}
                };
            }
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
        Name::from("Ball"),
    ));
}

fn move_ball(mut ball: Query<(&mut Transform, &Speed), With<Ball>>, time: Res<Time>) {
    for (mut transform, speed) in &mut ball {
        transform.translation.y += speed.0.y * time.delta_seconds();
        transform.translation.x -= speed.0.x * time.delta_seconds();
    }
}
