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

#[cfg(test)]
mod tests {
    use crate::ball::Ball;

    use super::*;

    #[test]
    fn should_decrease_lifes_counter_on_bounding_box_bottom_collision() {
        let mut app = App::new();
        app.insert_resource(Lives(3));
        app.add_event::<BallCollisionEvent>();

        app.add_systems(Update, decrease_lifes_counter);

        let bounding_box = app.world.spawn(BoundingBox).id();
        let ball = app.world.spawn(Ball { radius: 10.0 }).id();

        app.world
            .resource_mut::<Events<BallCollisionEvent>>()
            .send(BallCollisionEvent {
                ball,
                with: bounding_box,
                collision: Collision::Bottom,
            });

        app.update();

        assert_eq!(app.world.resource::<Lives>().0, 2);
    }

    #[test]
    fn should_set_state_to_game_over_on_0_lifes() {
        let mut app = App::new();
        app.insert_resource(Lives(1));
        app.add_state::<AppState>();
        app.add_event::<BallCollisionEvent>();

        let bounding_box = app.world.spawn(BoundingBox).id();
        let ball = app.world.spawn(Ball { radius: 10.0 }).id();

        app.add_systems(
            Update,
            (
                decrease_lifes_counter,
                game_over,
                apply_state_transition::<AppState>,
            )
                .chain(),
        );

        app.world
            .resource_mut::<Events<BallCollisionEvent>>()
            .send(BallCollisionEvent {
                ball,
                with: bounding_box,
                collision: Collision::Bottom,
            });

        app.update();

        let state = app.world.resource::<State<AppState>>().get();
        assert_eq!(*state, AppState::GameOver);
    }
}
