use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;

use crate::{
    ball::BallPlugin,
    block::BlockPlugin,
    config::ConfigPlugin,
    debug::DebugPlugin,
    paddle::{Dimensions, PaddlePlugin},
    ui::UiPlugin,
};

pub struct GamePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    PlayingBall,
    InGame,
    GameOver,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpawningSet {
    Paddle,
    Deferred,
    Ball,
    Blocks,
}

#[derive(Component)]
pub struct BoundingBox;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((
                DefaultPlugins,
                WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
                DebugLinesPlugin::default(),
                BallPlugin,
                ConfigPlugin,
                PaddlePlugin,
                UiPlugin,
                BlockPlugin,
                DebugPlugin,
            ))
            .configure_sets(
                OnEnter(GameState::PlayingBall),
                (
                    SpawningSet::Paddle,
                    SpawningSet::Deferred,
                    SpawningSet::Ball,
                    SpawningSet::Blocks
                )
                    .chain(),
            )
            .add_systems(Update, start_game.run_if(in_state(GameState::Menu)))
            .add_systems(
                OnEnter(GameState::PlayingBall),
                apply_deferred.in_set(SpawningSet::Deferred),
            )
            .add_systems(Startup, (spawn_camera, spawn_bounding_box))
            .add_systems(Update, print_state);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_bounding_box(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(window.width(), window.height())),
                color: Color::GRAY,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..Default::default()
        },
        Dimensions(Vec2::new(window.width(), window.height())),
        Name::from("Bounding box"),
        BoundingBox,
    ));
}

fn start_game(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Return) {
        state.set(GameState::PlayingBall)
    }
}

fn print_state(state: Res<State<GameState>>) {
    info!("{:?}", state)
}
