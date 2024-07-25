use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(60.0),
                            height: Val::Px(25.0),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    },
                    On::<Pointer<Click>>::run(|| info!("clicked!")),
                    On::<Pointer<Over>>::run(|| info!("hovered!")),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Start",
                            TextStyle {
                                font: asset_server.load("fonts/x12y12pxMaruMinya.ttf"),
                                font_size: 12.0,
                                color: Color::WHITE,
                            },
                        ),
                        Pickable::IGNORE,
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(60.0),
                            height: Val::Px(25.0),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    },
                    On::<Pointer<Click>>::run(|| info!("clicked!")),
                    On::<Pointer<Over>>::run(|| info!("hovered!")),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Exit",
                            TextStyle {
                                font: asset_server.load("fonts/x12y12pxMaruMinya.ttf"),
                                font_size: 12.0,
                                color: Color::WHITE,
                            },
                        ),
                        Pickable::IGNORE,
                    ));
                });
        });
}
