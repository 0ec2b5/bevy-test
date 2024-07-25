//! Provides sensible defaults for mouse picking inputs.

use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    render::camera::NormalizedRenderTarget,
};
use bevy_mod_picking::picking_core::{
    pointer::{InputMove, InputPress, Location, PointerButton, PointerId},
    PointerCoreBundle,
};

use crate::{canvas::CanvasHandle, prelude::*};

/// Spawns the default mouse pointer.
pub fn spawn_mouse_pointer(mut commands: Commands) {
    commands.spawn((
        PointerCoreBundle::new(PointerId::Mouse),
        #[cfg(feature = "selection")]
        bevy_mod_picking::selection::PointerMultiselect::default(),
    ));
}

/// Sends mouse pointer events to be processed by the core plugin
#[allow(clippy::too_many_arguments)]
pub fn mouse_pick_events(
    canvas_camera: Query<(&Camera, &GlobalTransform), With<CanvasCamera>>,
    window_camera: Query<(&Camera, &GlobalTransform), With<WindowCamera>>,
    canvas: Res<CanvasHandle>,
    // Input
    mut cursor_moves: EventReader<CursorMoved>,
    mut cursor_last: Local<Vec2>,
    mut mouse_inputs: EventReader<MouseButtonInput>,
    // Output
    mut pointer_move: EventWriter<InputMove>,
    mut pointer_presses: EventWriter<InputPress>,
) {
    let Ok((canvas_camera, canvas_transform)) = canvas_camera.get_single() else {
        return;
    };
    let Ok((window_camera, window_transform)) = window_camera.get_single() else {
        return;
    };

    for event in cursor_moves.read() {
        let position = if let Some(world_position) =
            window_camera.viewport_to_world_2d(window_transform, event.position)
        {
            if let Some(viewport_position) =
                canvas_camera.world_to_viewport(canvas_transform, world_position.extend(0.0))
            {
                viewport_position
            } else {
                continue;
            }
        } else {
            continue;
        };

        pointer_move.send(InputMove::new(
            PointerId::Mouse,
            Location {
                target: NormalizedRenderTarget::Image(canvas.clone()),
                position,
            },
            position - *cursor_last,
        ));
        *cursor_last = position;
    }

    for input in mouse_inputs.read() {
        let button = match input.button {
            MouseButton::Left => PointerButton::Primary,
            MouseButton::Right => PointerButton::Secondary,
            MouseButton::Middle => PointerButton::Middle,
            MouseButton::Other(_) => continue,
            MouseButton::Back => continue,
            MouseButton::Forward => continue,
        };

        match input.state {
            ButtonState::Pressed => {
                pointer_presses.send(InputPress::new_down(PointerId::Mouse, button));
            }
            ButtonState::Released => {
                pointer_presses.send(InputPress::new_up(PointerId::Mouse, button));
            }
        }
    }
}
