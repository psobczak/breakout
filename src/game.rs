use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;

use crate::{
    ball::BallPlugin,
    block::BlockPlugin,
    config::ConfigPlugin,
    debug::DebugPlugin,
    paddle::{Dimensions, PaddlePlugin},
    stats::StatsPlugin,
    ui::UiPlugin,
};

pub struct GamePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    AssetLoading,
    Menu,
    Playing,
    GameOver,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum PlayState {
    #[default]
    ReadyToShoot,
    BallInGame,
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
        app.add_state::<AppState>()
            .add_state::<PlayState>()
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
                StatsPlugin,
            ))
            .configure_sets(
                OnEnter(AppState::Playing),
                (
                    SpawningSet::Paddle,
                    SpawningSet::Deferred,
                    SpawningSet::Ball,
                    SpawningSet::Blocks,
                )
                    .chain(),
            )
            .add_systems(Update, start_game.run_if(in_state(AppState::Menu)))
            .add_systems(
                OnEnter(AppState::Playing),
                apply_deferred.in_set(SpawningSet::Deferred),
            )
            .add_systems(Startup, (spawn_camera, spawn_bounding_box));
            // .add_systems(OnExit(PlayState::BallInGame), systems);
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

fn start_game(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::Return) {
        state.set(AppState::Playing)
    }
}

pub fn despawn_with_component<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive()
    }
}