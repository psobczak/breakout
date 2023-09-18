use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    ball::Bounces,
    debug::{Drag, DragEvent, MousePosition},
    game::{despawn_with_component, GameState},
    stats::Lives,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup,))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnExit(GameState::Menu), despawn_with_component::<Menu>)
            .add_systems(
                Update,
                (update_bounce_counter.run_if(resource_changed::<Bounces>()),)
                    .distributive_run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    spawn_measuring_tape,
                    update_lifes_counter.run_if(resource_exists_and_changed::<Lives>()),
                    update_mouse_coordinates.run_if(resource_changed::<MousePosition>()),
                    (update_measuring_tape_length, despawn_measuring_tape)
                        .distributive_run_if(any_with_component::<MeasuringTape>()),
                ),
            );
    }
}

#[derive(Component)]
struct BounceCounter;

#[derive(Component)]
struct MouseCoordinates;

#[derive(Component)]
struct MeasuringTape;

#[derive(Component)]
struct BallCoordinates;

#[derive(Component)]
struct LifesCounter;

#[derive(Component)]
struct Menu;

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            Menu,
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::RED.into(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(50.0),
                    ..Default::default()
                },
                background_color: Color::GOLD.into(),
                ..Default::default()
            });
        });
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        BounceCounter,
    ));

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        LifesCounter,
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                }),
                MouseCoordinates,
            ));
        });
}

fn spawn_measuring_tape(
    mut commands: Commands,
    mut reader: EventReader<DragEvent>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    for event in reader.iter() {
        if let DragEvent::Start { viewport, .. } = event {
            let text_bundle = TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Vh((viewport.y / window.height()) * 100.0),
                left: Val::Vw((viewport.x / window.width()) * 100.0),
                ..default()
            });

            commands.spawn((text_bundle, MeasuringTape));
        }
    }
}

fn despawn_measuring_tape(
    mut reader: EventReader<DragEvent>,
    tape: Query<Entity, With<MeasuringTape>>,
    mut commands: Commands,
) {
    let tape = tape.single();
    for event in reader.iter() {
        if let DragEvent::Stop = event {
            commands.entity(tape).despawn_recursive()
        }
    }
}

fn update_bounce_counter(
    mut bounces_counter: Query<&mut Text, With<BounceCounter>>,
    bounces: Res<Bounces>,
) {
    let mut text = bounces_counter.single_mut();
    text.sections[0].value = bounces.0.to_string();
}

fn update_mouse_coordinates(
    mouse_postition: Res<MousePosition>,
    mut mouse_coordinates: Query<&mut Text, With<MouseCoordinates>>,
) {
    let mut text = mouse_coordinates.single_mut();
    text.sections[0].value = format!(
        "[x: {:.0}, y: {:.0}]",
        mouse_postition.world.x, mouse_postition.world.y
    );
}

fn update_measuring_tape_length(
    drag: Res<Drag>,
    mut measuring_tape: Query<&mut Text, With<MeasuringTape>>,
) {
    let mut text = measuring_tape.single_mut();
    if let Some(distance) = drag.distance() {
        text.sections[0].value = format!("{:.0}", distance);
    }
}

fn update_lifes_counter(
    lifes: Res<Lives>,
    mut lifes_counter: Query<&mut Text, With<LifesCounter>>,
) {
    let mut text = lifes_counter.single_mut();
    text.sections[0].value = format!("Lifes left: {}", lifes.0);
}
