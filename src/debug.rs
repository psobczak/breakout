use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_debug_lines::DebugLines;

use crate::game::GameState;

pub struct DebugPlugin;

#[derive(Resource, Default)]
pub struct MousePosition(pub Vec2);

#[derive(Resource, Default, Debug)]
struct Drag {
    start: Option<Vec2>,
    end: Option<Vec2>,
}

impl Drag {
    fn distance(&self) -> Option<f32> {
        if let Some(start) = self.start {
            if let Some(end) = self.end {
                return Some(start.distance(end));
            }
        }
        return None;
    }
}

fn handle_drag_event(
    input: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut drag: ResMut<Drag>,
) {
    if input.pressed(MouseButton::Left) {
        if drag.start.is_none() {
            drag.start = Some(mouse_position.0);
        }

        drag.end = Some(mouse_position.0);
    }

    if input.just_released(MouseButton::Left) {
        drag.start = None;
        drag.end = None;
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition::default())
            .insert_resource(Drag::default())
            .add_systems(Update, (update_mouse_position,))
            .add_systems(
                Update,
                (handle_drag_event, draw_measuring_tape)
                    .distributive_run_if(in_state(GameState::InGame)),
            );
    }
}

fn update_mouse_position(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    if let Some(cursor_postion) = window.cursor_position() {
        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_postion)
        {
            mouse_position.0 = world_position;
        }
    }
}

fn draw_measuring_tape(drag: Res<Drag>, mut lines: ResMut<DebugLines>) {
    let duration = 0.0;
    if let Some(start) = drag.start {
        if let Some(end) = drag.end {
            lines.line(start.extend(0.0), end.extend(0.0), duration);
            info!("{:?}", drag.distance())
        }
    }
}
