use bevy::prelude::*;

use crate::{ball::Bounces, debug::MousePosition, game::GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup,)).add_systems(
            Update,
            (
                update_bounce_counter.run_if(resource_changed::<Bounces>()),
                (update_mouse_coordinates,)
                    .distributive_run_if(resource_changed::<MousePosition>()),
            )
                .distributive_run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component)]
struct BounceCounter;

#[derive(Component)]
struct MouseCoordinates;

#[derive(Component)]
struct MeasuringTape;

fn setup(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 30.0,
                color: Color::WHITE,
                ..Default::default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        BounceCounter,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 20.0,
                color: Color::WHITE,
                ..Default::default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        MouseCoordinates,
    ));
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
        mouse_postition.0.x, mouse_postition.0.y
    );
}
