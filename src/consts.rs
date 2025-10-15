use bevy::prelude::*;
use bevy_egui::egui;
use bevy_smud::BlendMode;
use include_dir::include_dir;

pub const SIDE_PANEL_WIDTH: f32 = 550.0;
pub const DEFAULT_CAMERA_POSITION: Vec2 = {
    // Center in the remaining space not taken by the side panel
    Vec2::new(-SIDE_PANEL_WIDTH / 2.0, 0.0)
};
pub const DEFAULT_BACKGROUND_COLOR: egui::Color32 = {
    // Same as default ClearColor
    egui::Color32::from_rgb(43, 44, 47)
};
pub const DEFAULT_SHAPE_POSITION: Vec3 = Vec3::ZERO;
pub const DEFAULT_SHAPE_ROTATION: f32 = 0.0;
pub const DEFAULT_SHAPE_SCALE: f32 = 1.0;
pub const DEFAULT_SHAPE_BOUNDS_LENGTH: f32 = 500.0;
pub const DEFAULT_SHAPE_COLOR: egui::Color32 = egui::Color32::from_rgb(200, 100, 100);
pub const DEFAULT_SHAPE_PARAMS: Vec4 = Vec4::ZERO;
pub const DEFAULT_SHAPE_BLEND_MODE: BlendMode = BlendMode::Alpha;
pub const DEFAULT_SDF_TEMPLATE: &str = "circle";
pub const DEFAULT_FILL_TEMPLATE: &str = "cubic_falloff";
pub static SDF_TEMPLATE_DIR: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/sdf");
pub static FILL_TEMPLATE_DIR: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/fill");
