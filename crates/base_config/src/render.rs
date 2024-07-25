use bevy::render::view::RenderLayers;

pub const CANVAS_LAYER: RenderLayers = RenderLayers::layer(0);
pub const WINDOW_LAYER: RenderLayers = RenderLayers::layer(1);
pub const POINTER_LAYER: RenderLayers = RenderLayers::layer(2);
pub const CANVAS_ORDER: isize = -2;
pub const POINTER_ORDER: isize = -1;
pub const WINDOW_ORDER: isize = 0;
