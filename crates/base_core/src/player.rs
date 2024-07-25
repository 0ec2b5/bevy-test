use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Shoot,
}

fn setup(mut commands: Commands, config: Res<BaseShapeConfig>) {
    commands.spawn((
        Player,
        ShapeBundle::circle(
            &ShapeConfig {
                color: Color::WHITE,
                texture: None,
                render_layers: None,
                ..config.0
            },
            15.0,
        ),
        InputManagerBundle::with_map(InputMap::new([
            (PlayerAction::Up, KeyCode::ArrowUp),
            (PlayerAction::Up, KeyCode::KeyW),
            (PlayerAction::Down, KeyCode::ArrowDown),
            (PlayerAction::Down, KeyCode::KeyS),
            (PlayerAction::Left, KeyCode::ArrowLeft),
            (PlayerAction::Left, KeyCode::KeyA),
            (PlayerAction::Right, KeyCode::ArrowRight),
            (PlayerAction::Right, KeyCode::KeyD),
            (PlayerAction::Shoot, KeyCode::KeyZ),
            (PlayerAction::Shoot, KeyCode::KeyJ),
        ])),
    ));
}

fn movement(mut player_query: Query<(&mut Transform, &ActionState<PlayerAction>), With<Player>>) {
    let (mut transform, action) = player_query.single_mut();

    if action.pressed(&PlayerAction::Up) {
        transform.translation.y += 3.0;
    }
    if action.pressed(&PlayerAction::Down) {
        transform.translation.y -= 3.0;
    }
    if action.pressed(&PlayerAction::Left) {
        transform.translation.x -= 3.0;
    }
    if action.pressed(&PlayerAction::Right) {
        transform.translation.x += 3.0;
    }
}
