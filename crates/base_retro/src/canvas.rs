use base_config::prelude::*;
use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{BevyDefault, ImageSampler},
        view::{Msaa, RenderLayers},
    },
    window::WindowResized,
};
use bevy_mod_picking::prelude::*;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CanvasConfig::default())
            .init_resource::<CanvasHandle>()
            .insert_resource(Msaa::Off)
            .add_systems(Startup, setup_canvas.in_set(CanvasSetup))
            .add_systems(
                PreUpdate,
                (
                    fit_to_window,
                    apply_config,
                    lock_cursor.run_if(|retro_window: Res<CanvasConfig>| retro_window.lock_cursor),
                ),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanvasSetup;

pub enum CanvasScale {
    AutoFit { pixel_perfect: bool },
    Manual(f32),
}

impl Default for CanvasScale {
    fn default() -> Self {
        Self::AutoFit {
            pixel_perfect: false,
        }
    }
}

#[derive(Resource)]
pub struct CanvasConfig {
    pub resolution: Vec2,
    pub scale: CanvasScale,
    pub lock_cursor: bool,
    pub clear_color: ClearColorConfig,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            resolution: Vec2::new(160.0, 144.0),
            scale: CanvasScale::default(),
            lock_cursor: false,
            clear_color: default(),
        }
    }
}

impl CanvasConfig {
    pub fn resolve_scale(&self, width: f32, height: f32) -> f32 {
        match self.scale {
            CanvasScale::Manual(scale) => scale,
            CanvasScale::AutoFit { pixel_perfect } => {
                let scale = if (width / height) < 1.0 {
                    width / self.resolution.x
                } else {
                    height / self.resolution.y
                };

                1.0 / if pixel_perfect {
                    let int = scale as u32;
                    let next = (int).next_power_of_two();
                    ((if next == int { int } else { next / 2 }) as f32).max(1.0)
                } else {
                    scale
                }
            }
        }
    }
}

#[derive(Component)]
pub struct WindowCamera;

#[derive(Component)]
pub struct CanvasCamera;

#[derive(Resource, Deref, DerefMut)]
pub struct CanvasHandle(Handle<Image>);

impl FromWorld for CanvasHandle {
    fn from_world(world: &mut World) -> Self {
        let config = world.resource::<CanvasConfig>();

        let size = Extent3d {
            width: config.resolution.x as u32,
            height: config.resolution.y as u32,
            ..default()
        };

        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::bevy_default(),
                usage: TextureUsages::COPY_DST
                    | TextureUsages::TEXTURE_BINDING
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            sampler: ImageSampler::nearest(),
            ..default()
        };

        image.resize(size);

        let canvas = world.resource_mut::<Assets<Image>>().add(image);

        Self(canvas)
    }
}

#[derive(Component)]
pub struct CanvasSprite;

fn setup_canvas(
    mut commands: Commands,
    config: Res<CanvasConfig>,
    canvas: Res<CanvasHandle>,
    windows: Query<&Window>,
) {
    let window = windows.single();

    debug_assert_eq!(CANVAS_LAYER, RenderLayers::layer(0));

    commands.spawn((
        CanvasCamera,
        CANVAS_LAYER,
        Camera2dBundle {
            camera: Camera {
                order: CANVAS_ORDER,
                target: RenderTarget::Image(canvas.clone()),
                clear_color: config.clear_color,
                ..default()
            },
            ..default()
        },
        IsDefaultUiCamera,
    ));

    commands.spawn((
        CanvasSprite,
        WINDOW_LAYER,
        SpriteBundle {
            texture: canvas.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ..default()
        },
        Pickable::IGNORE,
    ));

    commands.spawn((
        WindowCamera,
        WINDOW_LAYER,
        Camera2dBundle {
            camera: Camera {
                order: WINDOW_ORDER,
                ..default()
            },
            projection: OrthographicProjection {
                scale: config.resolve_scale(window.resolution.width(), window.resolution.height()),
                ..Camera2dBundle::default().projection
            },
            ..default()
        },
    ));
}

fn apply_config(
    config: Res<CanvasConfig>,
    mut handles: Query<&Handle<Image>, With<CanvasSprite>>,
    mut projections: Query<&mut OrthographicProjection, With<WindowCamera>>,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
) {
    if !config.is_changed() {
        return;
    }

    let window = windows.single();
    let handle = handles.single_mut();
    let image = images.get_mut(handle).unwrap();

    let size = Extent3d {
        width: config.resolution.x as u32,
        height: config.resolution.y as u32,
        ..default()
    };
    image.resize(size);

    let mut projection = projections.single_mut();
    projection.scale = config.resolve_scale(window.resolution.width(), window.resolution.height());
}

fn fit_to_window(
    mut resize_reader: EventReader<WindowResized>,
    config: Res<CanvasConfig>,
    mut projections: Query<&mut OrthographicProjection, With<WindowCamera>>,
) {
    for resize in resize_reader.read() {
        let mut projection = projections.single_mut();
        projection.scale = config.resolve_scale(resize.width, resize.height);
    }
}

fn lock_cursor(
    config: Res<CanvasConfig>,
    mut windows: Query<&mut Window>,
    handles: Query<&Transform, With<CanvasSprite>>,
) {
    let mut window = windows.single_mut();
    let transform = handles.single();

    let scale = transform.scale;
    let width = window.width();
    let height = window.height();

    if let Some(cursor) = window.cursor_position() {
        window.set_cursor_position(Some(Vec2::new(
            cursor.x.clamp(
                (width - config.resolution.x * scale.x) / 2.0,
                (width + config.resolution.x * scale.x) / 2.0,
            ),
            cursor.y.clamp(
                (height - config.resolution.y * scale.y) / 2.0,
                (height + config.resolution.y * scale.y) / 2.0,
            ),
        )));
    }
}
