use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct QuickClosePlugin;

impl Plugin for QuickClosePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<WindowAction>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, close_window);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum WindowAction {
    Kill,
}

fn setup(mut commands: Commands) {
    commands.spawn(InputManagerBundle::with_map(InputMap::new([(
        WindowAction::Kill,
        UserInput::modified(Modifier::Control, KeyCode::KeyC),
    )])));
}

fn close_window(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    action_state: Query<&ActionState<WindowAction>>,
) {
    for (window, focus) in &focused_windows {
        if !focus.focused {
            continue;
        }

        for action in &action_state {
            if action.just_pressed(&WindowAction::Kill) {
                commands.entity(window).despawn()
            }
        }
    }
}
