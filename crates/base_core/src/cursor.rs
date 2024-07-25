use base_config::prelude::*;
use base_retro::{canvas::CanvasSetup, prelude::*};
use bevy::{prelude::*, sprite::Anchor};
use bevy_mod_picking::{
    pointer::{InputMove, InputPress, Location, PressDirection},
    prelude::*,
};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorConfig>()
            .add_systems(Startup, setup.after(CanvasSetup))
            .add_systems(Update, (update_position, update_icon, apply_icon));
    }
}

#[derive(Component)]
struct Cursor;

#[derive(Component)]
struct CursorCamera;

#[derive(Component, Default, PartialEq)]
pub enum CursorState {
    #[default]
    Arrow,
    Grab,
    Grabbing,
    Crosshair,
}

impl CursorState {
    fn get_data<'a>(&self, config: &'a CursorConfig) -> &'a CursorSprite {
        match self {
            CursorState::Arrow => &config.arrow,
            CursorState::Grab => &config.grab,
            CursorState::Grabbing => &config.grabbing,
            CursorState::Crosshair => &config.crosshair,
        }
    }
}

pub struct CursorSprite {
    texture: Handle<Image>,
    anchor: Anchor,
}

#[derive(Resource)]
pub struct CursorConfig {
    arrow: CursorSprite,
    grab: CursorSprite,
    grabbing: CursorSprite,
    crosshair: CursorSprite,
}

impl FromWorld for CursorConfig {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            arrow: CursorSprite {
                texture: asset_server.load("arrow.png"),
                anchor: Anchor::Custom(Vec2::new(-0.3, 0.5)),
            },
            grab: CursorSprite {
                texture: asset_server.load("grab.png"),
                anchor: Anchor::Custom(Vec2::new(-0.1, 0.2)),
            },
            grabbing: CursorSprite {
                texture: asset_server.load("grabbing.png"),
                anchor: Anchor::Custom(Vec2::new(-0.1, 0.2)),
            },
            crosshair: CursorSprite {
                texture: asset_server.load("crosshair.png"),
                anchor: Anchor::Center,
            },
        }
    }
}

#[derive(Component)]
pub struct Draggable;

fn setup(
    mut commands: Commands,
    config: Res<CursorConfig>,
    mut windows: Query<&mut Window>,
    camera: Query<&Camera, With<CanvasCamera>>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;

    commands.spawn((
        CursorCamera,
        POINTER_LAYER,
        Camera2dBundle {
            camera: Camera {
                order: POINTER_ORDER,
                target: camera.single().target.clone(),
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
    ));

    let state = CursorState::default();
    let cursor = state.get_data(config.as_ref());
    commands.spawn((
        Cursor,
        state,
        SpriteBundle {
            sprite: Sprite {
                anchor: cursor.anchor,
                ..default()
            },
            texture: cursor.texture.clone(),
            ..default()
        },
        Pickable::IGNORE,
        POINTER_LAYER,
    ));
}

fn update_position(
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    mut input_move: EventReader<InputMove>,
) {
    let mut cursor = cursor_query.single_mut();

    for InputMove {
        location: Location { position, .. },
        ..
    } in input_move.read()
    {
        cursor.translation.x = position.x;
        cursor.translation.y = position.y;
    }
}

#[allow(clippy::too_many_arguments)]
fn update_icon(
    mut input_move: EventReader<InputMove>,
    mut input_press: EventReader<InputPress>,
    pointer_map: Res<PointerMap>,
    mut current_drag: Local<Option<Entity>>,
    draggable: Query<(Has<Draggable>, Option<&Parent>)>,
    pointer_query: Query<&PointerInteraction>,
    mut states: Query<&mut CursorState, With<Cursor>>,
) {
    let mut icon = states.single_mut();

    for event in input_move.read() {
        let pointer_entity = pointer_map.get_entity(event.pointer_id).unwrap();
        let interaction = pointer_query.get(pointer_entity).unwrap();

        let mut hover = false;
        if let Some(&(mut entity, _)) = interaction.get_nearest_hit() {
            while let Ok((draggable, parent)) = draggable.get(entity) {
                if draggable {
                    if current_drag.is_none() {
                        icon.set_if_neq(CursorState::Grab);
                    }
                    hover = true;
                    break;
                }

                if let Some(parent) = parent {
                    entity = parent.get();
                } else {
                    break;
                }
            }
        }

        if current_drag.is_none() && !hover {
            icon.set_if_neq(CursorState::Arrow);
        }
    }

    for event in input_press.read() {
        if let PressDirection::Up = event.direction {
            *current_drag = None;
            icon.set_if_neq(CursorState::Arrow);
        };

        let pointer_entity = pointer_map.get_entity(event.pointer_id).unwrap();
        let interaction = pointer_query.get(pointer_entity).unwrap();

        if let Some(&(mut entity, _)) = interaction.get_nearest_hit() {
            while let Ok((draggable, parent)) = draggable.get(entity) {
                if draggable {
                    icon.set_if_neq(if let PressDirection::Down = event.direction {
                        *current_drag = Some(entity);
                        CursorState::Grabbing
                    } else {
                        *current_drag = None;
                        CursorState::Grab
                    });
                    break;
                }

                if let Some(parent) = parent {
                    entity = parent.get();
                } else {
                    break;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn apply_icon(
    config: Res<CursorConfig>,
    mut pointer: Query<(Ref<CursorState>, &mut Handle<Image>, &mut Sprite), With<Cursor>>,
) {
    let (state, mut handle, mut sprite) = pointer.single_mut();
    if !state.is_changed() {
        return;
    }
    let cursor = state.get_data(config.as_ref());
    *handle = cursor.texture.clone();
    sprite.anchor = cursor.anchor;
}
