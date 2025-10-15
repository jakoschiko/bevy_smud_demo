use bevy::prelude::*;
use bevy_egui::egui;
use bevy_smud::{BlendMode, SmudShape};

use crate::{
    shader::create_shader,
    state::{GlobalState, ShaderKind, ShapeState},
    templates::Templates,
    util::convert_color,
};

pub fn add_shape(
    commands: &mut Commands,
    templates: &Templates,
    state: &mut GlobalState,
    shaders: &mut Assets<Shader>,
) {
    let mut transform = Transform::default();
    let mut shape = SmudShape::default();

    let shape_state = ShapeState {
        id: state.create_shape(),
        position: Vec3::ZERO,
        rotation: 0.0,
        scale: 1.0,
        bounds_length: 1000.0,
        color: egui::Color32::from_rgb(200, 100, 100),
        selected_shader: ShaderKind::Sdf,
        sdf_code: templates
            .default_template(ShaderKind::Sdf)
            .map(|t| t.code.clone())
            .unwrap_or_default(),
        fill_code: templates
            .default_template(ShaderKind::Fill)
            .map(|t| t.code.clone())
            .unwrap_or_default(),
        params: Vec4::ZERO,
        blend_mode: BlendMode::default(),
    };

    update_shape(
        state,
        shaders,
        &mut transform,
        &mut shape,
        &shape_state,
        true,
    );

    commands.spawn((transform, shape, shape_state));
}

pub fn clone_shape(
    commands: &mut Commands,
    state: &mut GlobalState,
    transform: &Transform,
    shape: &SmudShape,
    shape_state: &ShapeState,
) {
    let mut shape_state = shape_state.clone();
    shape_state.id = state.create_shape();

    commands.spawn((*transform, shape.clone(), shape_state));
}

pub fn update_shape(
    global_state: &mut GlobalState,
    shaders: &mut Assets<Shader>,
    transform: &mut Transform,
    shape: &mut SmudShape,
    shape_state: &ShapeState,
    compile_shader: bool,
) {
    *transform = Transform::from_translation(shape_state.position)
        .with_rotation(Quat::from_rotation_z(shape_state.rotation))
        .with_scale(Vec3::splat(shape_state.scale));

    shape.color = convert_color(shape_state.color);
    shape.bounds = Rectangle::from_length(shape_state.bounds_length);
    shape.params = shape_state.params;
    shape.blend_mode = shape_state.blend_mode;

    if compile_shader {
        shape.sdf = create_shader(shaders, global_state, &shape_state.sdf_code);
        shape.fill = create_shader(shaders, global_state, &shape_state.fill_code);
    }
}
