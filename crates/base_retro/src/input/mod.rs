use bevy::prelude::*;
use bevy_mod_picking::picking_core::PickSet;

pub mod mouse;
pub mod touch;

pub struct RetroInputPlugin;
impl Plugin for RetroInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RetroInputPluginSettings>()
            .add_systems(Startup, mouse::spawn_mouse_pointer)
            .add_systems(
                First,
                (
                    touch::touch_pick_events.run_if(RetroInputPluginSettings::is_touch_enabled),
                    mouse::mouse_pick_events.run_if(RetroInputPluginSettings::is_mouse_enabled),
                    // IMPORTANT: the commands must be flushed after `touch_pick_events` is run
                    // because we need pointer spawning to happen immediately to prevent issues
                    // with missed events during drag and drop.
                    apply_deferred,
                )
                    .chain()
                    .in_set(PickSet::Input),
            )
            .add_systems(
                Last,
                touch::deactivate_touch_pointers.run_if(RetroInputPluginSettings::is_touch_enabled),
            )
            .register_type::<RetroInputPluginSettings>();
    }
}

/// A resource used to enable and disable features of the [`RetroInputPlugin`].
///
/// [`bevy_mod_picking::picking_core::PickingPluginsSettings::is_input_enabled`]
/// can be used to toggle whether the core picking plugin processes the inputs
/// sent by this, or other input plugins, in one place.
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default)]
pub struct RetroInputPluginSettings {
    /// Should touch inputs be updated?
    is_touch_enabled: bool,
    /// Should mouse inputs be updated?
    is_mouse_enabled: bool,
}

impl Default for RetroInputPluginSettings {
    fn default() -> Self {
        Self {
            is_touch_enabled: true,
            is_mouse_enabled: true,
        }
    }
}

impl RetroInputPluginSettings {
    fn is_touch_enabled(state: Res<Self>) -> bool {
        state.is_touch_enabled
    }
    fn is_mouse_enabled(state: Res<Self>) -> bool {
        state.is_mouse_enabled
    }
}
