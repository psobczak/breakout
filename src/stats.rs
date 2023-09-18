use bevy::{prelude::*, sprite::collide_aabb::Collision};

use crate::{
    ball::BallCollisionEvent,
    config::{Config, GameConfig},
    game::{AppState, BoundingBox},
};

#[derive(Resource)]
pub struct Lives(pub u32);

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), insert_lifes)
            .add_systems(
                Update,
                (decrease_lifes_counter, game_over)
                    .distributive_run_if(in_state(AppState::Playing)),
            );
    }
}

fn insert_lifes(mut commands: Commands, game_config: Res<GameConfig>, assets: Res<Assets<Config>>) {
    let Some(config) = assets.get(&game_config.config) else {
        panic!("game config could not be loaded")
    };

    commands.insert_resource(Lives(config.stats.lifes));
}

fn decrease_lifes_counter(
    mut lifes: ResMut<Lives>,
    mut reader: EventReader<BallCollisionEvent>,
    bounding_box: Query<With<BoundingBox>>,
) {
    for event in reader.iter() {
        if bounding_box.get(event.with).is_ok() && event.collision == Collision::Bottom {
            lifes.0 -= 1;
        }
    }
}

fn game_over(mut state: ResMut<NextState<AppState>>, lifes: Res<Lives>) {
    if lifes.0 == 0 {
        state.set(AppState::GameOver)
    }
}
