use bevy::prelude::*;
use bevy_defer::{
    cancellation::SyncCancellation, tween::Playback, AsyncAccess, AsyncCommandsExtension,
    AsyncWorld,
};
use bevy_vector_shapes::prelude::*;
use interpolation::Ease;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
struct Enemy;

fn setup(mut commands: Commands, config: Res<BaseShapeConfig>, cancel: Local<SyncCancellation>) {
    let enemy = commands
        .spawn((
            Enemy,
            ShapeBundle::rect(
                &ShapeConfig {
                    color: Color::WHITE,
                    corner_radii: Vec4::splat(5.0),
                    texture: None,
                    render_layers: None,
                    ..config.0
                },
                Vec2::splat(32.0),
            ),
        ))
        .id();

    commands.spawn_task({
        let cancel = cancel.clone();
        move || async move {
            let transform = AsyncWorld.entity(enemy).component::<Transform>();
            let duration = 1.0;
            AsyncWorld.spawn(transform.interpolate(
                move |x| Vec3::new(-75.0, 0.0, 0.0).lerp(Vec3::new(75.0, 0.0, 0.0), x),
                |t, v| t.translation = v,
                f32::back_in_out,
                duration,
                Playback::Bounce,
                &cancel,
            ));
            AsyncWorld.spawn(transform.interpolate(
                move |x| 0.0_f32.lerp(-360.0, x),
                |t, v| t.rotation = Quat::from_rotation_z(v.to_radians()),
                f32::back_in_out,
                duration,
                Playback::Bounce,
                &cancel,
            ));

            AsyncWorld.sleep(duration * 5.0).await;
            cancel.cancel();

            Ok(())
        }
    });
}
