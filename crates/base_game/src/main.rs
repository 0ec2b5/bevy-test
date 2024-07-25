use avian2d::prelude::*;
use base_config::prelude::*;
use base_core::prelude::*;
use base_retro::{
    canvas::{CanvasConfig, CanvasScale},
    prelude::*,
};
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_defer::AsyncPlugin;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::prelude::*;

fn main() {
    let mut app = App::new();

    if cfg!(not(debug_assertions)) {
        app.add_plugins(bevy_embedded_assets::EmbeddedAssetPlugin {
            mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
        });
    }

    if cfg!(all(debug_assertions, not(target_arch = "wasm32"))) {
        app.add_plugins(QuickClosePlugin);
    }

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (CANVAS_SIZE * CANVAS_SCALE).into(),
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        PhysicsPlugins::default(),
        DefaultPickingPlugins
            .build()
            .disable::<InputPlugin>()
            .add(RetroInputPlugin),
        Shape2dPlugin::new(ShapeConfig {
            disable_laa: true,
            hollow: true,
            color: Color::WHITE,
            thickness: 1.0,
            thickness_type: ThicknessType::Pixels,
            ..ShapeConfig::default_2d()
        }),
        AsyncPlugin::default_settings(),
    ))
    .add_plugins((CanvasPlugin, MenuPlugin, PlayerPlugin, EnemyPlugin))
    .insert_resource(DebugPickingMode::Disabled)
    .insert_resource(Gravity(Vec2::NEG_Y * 500.0))
    .insert_resource(CanvasConfig {
        resolution: CANVAS_SIZE,
        scale: CanvasScale::AutoFit {
            pixel_perfect: true,
        },
        clear_color: ClearColorConfig::Custom(Color::BLACK),
        ..default()
    })
    .run();
}
