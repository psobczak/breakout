use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{ball::BallPlugin, config::ConfigPlugin, paddle::PaddlePlugin};

pub struct GamePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    Game,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((
                DefaultPlugins,
                WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
                BallPlugin,
                ConfigPlugin,
                PaddlePlugin,
            ))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
