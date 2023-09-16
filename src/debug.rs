use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_debug_lines::DebugLines;

use crate::game::GameState;

pub struct DebugPlugin;

#[derive(Resource, Default)]
pub struct MousePosition {
    pub world: Vec2,
    pub viewport: Vec2,
}

#[derive(Resource, Default, Debug)]
pub struct Drag {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
}

#[derive(Event)]
pub enum DragEvent {
    Start { world: Vec2, viewport: Vec2 },
    Dragging(Vec2),
    Stop,
}

impl Drag {
    pub fn distance(&self) -> Option<f32> {
        if let Some(start) = self.start {
            if let Some(end) = self.end {
                return Some(start.distance(end));
            }
        }
        return None;
    }
}

fn handle_drag(mut reader: EventReader<DragEvent>, mut drag: ResMut<Drag>) {
    for event in reader.iter() {
        match event {
            DragEvent::Start { world, .. } => {
                drag.start = Some(*world);
            }
            DragEvent::Dragging(position) => {
                drag.end = Some(*position);
            }
            DragEvent::Stop => {
                drag.start = None;
                drag.end = None;
            }
        }
    }
}

fn send_drag_event(
    input: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut writer: EventWriter<DragEvent>,
) {
    if input.just_pressed(MouseButton::Left) {
        writer.send(DragEvent::Start {
            world: mouse_position.world,
            viewport: mouse_position.viewport,
        });
    }

    if input.pressed(MouseButton::Left) {
        writer.send(DragEvent::Dragging(mouse_position.world));
    }

    if input.just_released(MouseButton::Left) {
        writer.send(DragEvent::Stop);
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition::default())
            .insert_resource(Drag::default())
            .add_event::<DragEvent>()
            .add_systems(
                Update,
                (
                    send_drag_event,
                    handle_drag,
                    draw_measuring_tape,
                    update_mouse_position,
                )
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
        mouse_position.viewport = cursor_postion;
        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_postion)
        {
            mouse_position.world = world_position;
        }
    }
}

fn draw_measuring_tape(drag: Res<Drag>, mut lines: ResMut<DebugLines>) {
    let duration = 0.0;
    if let Some(start) = drag.start {
        if let Some(end) = drag.end {
            lines.line(start.extend(0.0), end.extend(0.0), duration);
        }
    }
}
