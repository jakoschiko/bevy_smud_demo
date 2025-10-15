use include_dir::include_dir;

pub const SIDE_PANEL_WIDTH: f32 = 550.0;
pub const DEFAULT_SDF_TEMPLATE: &str = "circle";
pub const DEFAULT_FILL_TEMPLATE: &str = "cubic_falloff";
pub static SDF_TEMPLATE_DIR: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/sdf");
pub static FILL_TEMPLATE_DIR: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/fill");
