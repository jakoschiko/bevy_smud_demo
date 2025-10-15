mod consts;
mod shader;
mod shape;
mod state;
mod templates;
mod util;

use std::{collections::BTreeSet, f32::consts::TAU};

use bevy::{picking::hover::PickingInteraction, prelude::*};
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, Widget},
};
use bevy_smud::prelude::*;

use crate::{
    shape::{add_shape, clone_shape, update_shape},
    state::{GlobalState, SelectedTab, ShaderKind, ShapeState},
    templates::Templates,
    util::convert_color,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Smud Demo".into(),
                #[cfg(all(target_family = "wasm", target_os = "unknown"))]
                fit_canvas_to_parent: true, // We need this to fill the entire web page
                #[cfg(all(target_family = "wasm", target_os = "unknown"))]
                prevent_default_event_handling: false, // We need this for copy/paste
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SmudPlugin)
        .add_plugins(SmudPickingPlugin)
        .add_plugins(EguiPlugin::default())
        .insert_resource(Templates::default())
        .insert_resource(GlobalState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(EguiPrimaryContextPass, gui)
        .run();
}

#[derive(Component)]
struct ShapeCamera;

fn setup(
    mut commands: Commands,
    templates: Res<Templates>,
    mut global_state: ResMut<GlobalState>,
    mut clear_color: ResMut<ClearColor>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    // Initialize background
    clear_color.0 = convert_color(global_state.background_color);

    // Initialize camera
    commands.spawn((
        ShapeCamera,
        Camera2d,
        Msaa::Off,
        Transform::from_translation(global_state.camera_position.extend(0.0)),
    ));

    // Initialize shape
    add_shape(&mut commands, &templates, &mut global_state, &mut shaders);
}

fn update(
    mut global_state: ResMut<GlobalState>,
    mut clear_color: ResMut<ClearColor>,
    picking_query: Query<(&ShapeState, &PickingInteraction), Changed<PickingInteraction>>,
    mut camera_query: Single<&mut Transform, With<ShapeCamera>>,
) {
    // Update background
    clear_color.0 = convert_color(global_state.background_color);

    // Update camera
    let camera_transform = camera_query.as_mut();
    *camera_transform = Transform::from_translation(global_state.camera_position.extend(0.0));

    // Pick shape
    for (shape_state, &interaction) in picking_query {
        if interaction == PickingInteraction::Pressed {
            global_state.selected_tab = SelectedTab::Shape(shape_state.id);
        }
    }
}

fn gui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    templates: Res<Templates>,
    mut global_state: ResMut<GlobalState>,
    mut shaders: ResMut<Assets<Shader>>,
    mut shape_query: Query<(Entity, &mut Transform, &mut SmudShape, &mut ShapeState)>,
) -> Result {
    // Build UI
    egui::SidePanel::left("side_panel")
        .default_width(consts::SIDE_PANEL_WIDTH)
        .show(contexts.ctx_mut()?, |ui| {
            ui.add_space(4.0);

            // UI for selecting/editing tabs
            tab_bar(
                ui,
                &mut commands,
                &templates,
                &mut global_state,
                &mut shaders,
                &shape_query,
            );

            ui.separator();

            match global_state.selected_tab {
                SelectedTab::Global => {
                    // UI for changing global settings
                    global_settings(ui, &mut global_state);
                }
                SelectedTab::Shape(id) => {
                    // UI for changing the selected shape
                    if let Some((_, mut transform, mut shape, mut shape_state)) = shape_query
                        .iter_mut()
                        .find(|(_, _, _, shape_state)| shape_state.id == id)
                    {
                        // UI for changing non-shader shape settings
                        shape_settings(ui, &mut shape_state);

                        ui.separator();

                        // UI for editing the shader shape
                        let compile_shader = shader_editor(ui, &templates, &mut shape_state);

                        // Apply changes
                        update_shape(
                            &mut global_state,
                            &mut shaders,
                            &mut transform,
                            &mut shape,
                            &shape_state,
                            compile_shader,
                        );
                    }
                }
            };
        });

    Ok(())
}

fn tab_bar(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    templates: &Templates,
    global_state: &mut GlobalState,
    shaders: &mut Assets<Shader>,
    shape_query: &Query<(Entity, &mut Transform, &mut SmudShape, &mut ShapeState)>,
) {
    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut global_state.selected_tab,
            SelectedTab::Global,
            "Global",
        );

        ui.separator();

        if ui.button("Add").clicked() {
            add_shape(commands, templates, global_state, shaders);
        }

        let shapes: BTreeSet<_> = shape_query
            .iter()
            .map(|(_, _, _, shape_state)| shape_state.id)
            .collect();

        let selected_shape = match global_state.selected_tab {
            SelectedTab::Shape(id) => Some(id),
            _ => None,
        };

        ui.add_enabled_ui(selected_shape.is_some(), |ui| {
            if ui.button("Copy").clicked()
                && let Some(id) = selected_shape
                && let Some((transform, shape, shape_state)) =
                    shape_query
                        .iter()
                        .find_map(|(_, transform, shape, shape_state)| {
                            (shape_state.id == id).then_some((transform, shape, shape_state))
                        })
            {
                clone_shape(commands, global_state, transform, shape, shape_state);
            }

            if ui.button("Delete").clicked()
                && let Some(id) = selected_shape
                && let Some(entity) = shape_query.iter().find_map(|(entity, _, _, shape_state)| {
                    (shape_state.id == id).then_some(entity)
                })
            {
                let neighbor_id = shapes
                    .range(0..id)
                    .next_back()
                    .copied()
                    .or_else(|| shapes.range(id + 1..).next().copied());
                global_state
                    .select_tab(neighbor_id.map_or(SelectedTab::Global, SelectedTab::Shape));
                commands.entity(entity).despawn();
            }
        });

        egui::ScrollArea::horizontal()
            .id_salt("scroll_tab")
            .show(ui, |ui| {
                for id in shapes {
                    let selector = ui.selectable_value(
                        &mut global_state.selected_tab,
                        SelectedTab::Shape(id),
                        format!("shape_{id}"),
                    );
                    if global_state.check_scroll_to(id) {
                        selector.scroll_to_me(None);
                    }
                }
            });
    });
}

fn global_settings(ui: &mut egui::Ui, global_state: &mut GlobalState) {
    egui::Grid::new("grid_global")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Camera position:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    global_state.camera_position = consts::DEFAULT_CAMERA_POSITION;
                };
                ui.label("x");
                egui::DragValue::new(&mut global_state.camera_position.x)
                    .speed(5.0)
                    .ui(ui);
                ui.label("y");
                egui::DragValue::new(&mut global_state.camera_position.y)
                    .speed(5.0)
                    .ui(ui);
            });
            ui.end_row();

            ui.label("Background color:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    global_state.background_color = consts::DEFAULT_BACKGROUND_COLOR;
                };
                ui.color_edit_button_srgba(&mut global_state.background_color);
            });
            ui.end_row();
        });
}

fn shape_settings(ui: &mut egui::Ui, shape_state: &mut ShapeState) {
    egui::Grid::new("grid_shape")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Position:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    shape_state.position = consts::DEFAULT_SHAPE_POSITION;
                };
                ui.label("x");
                egui::DragValue::new(&mut shape_state.position.x)
                    .speed(5.0)
                    .ui(ui);
                ui.label("y");
                egui::DragValue::new(&mut shape_state.position.y)
                    .speed(5.0)
                    .ui(ui);
                ui.label("z");
                egui::DragValue::new(&mut shape_state.position.z)
                    .speed(1.0)
                    .ui(ui);
            });
            ui.end_row();

            ui.label("Rotation:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    shape_state.rotation = consts::DEFAULT_SHAPE_ROTATION;
                };
                ui.add(
                    egui::DragValue::new(&mut shape_state.rotation)
                        .min_decimals(2)
                        .speed(TAU / 50.0),
                );
            });
            ui.end_row();

            ui.label("Scale:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    shape_state.scale = consts::DEFAULT_SHAPE_SCALE;
                };
                ui.add(
                    egui::DragValue::new(&mut shape_state.scale)
                        .min_decimals(1)
                        .speed(1.0 / 5.0),
                );
            });
            ui.end_row();

            ui.label("Color:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    shape_state.color = consts::DEFAULT_SHAPE_COLOR;
                };
                ui.color_edit_button_srgba(&mut shape_state.color);
            });
            ui.end_row();

            ui.label("Bounds length:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    shape_state.bounds_length = consts::DEFAULT_SHAPE_BOUNDS_LENGTH;
                };
                egui::Slider::new(&mut shape_state.bounds_length, 0.0..=2000.0).ui(ui);
            });
            ui.end_row();

            ui.label("Params:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    shape_state.params = consts::DEFAULT_SHAPE_PARAMS;
                };
                ui.label("x");
                egui::DragValue::new(&mut shape_state.params.x)
                    .speed(1.0)
                    .ui(ui);
                ui.label("y");
                egui::DragValue::new(&mut shape_state.params.y)
                    .speed(1.0)
                    .ui(ui);
                ui.label("z");
                egui::DragValue::new(&mut shape_state.params.z)
                    .speed(1.0)
                    .ui(ui);
                ui.label("w");
                egui::DragValue::new(&mut shape_state.params.w)
                    .speed(1.0)
                    .ui(ui);
            });
            ui.end_row();

            ui.label("Blend mode:");
            ui.horizontal(|ui| {
                if ui.button("⟲").clicked() {
                    shape_state.blend_mode = consts::DEFAULT_SHAPE_BLEND_MODE;
                };
                egui::ComboBox::from_id_salt("blend_mode")
                    .selected_text(format!("{:?}", shape_state.blend_mode))
                    .show_ui(ui, |ui| {
                        for blend_mode in [BlendMode::Alpha, BlendMode::Additive] {
                            ui.selectable_value(
                                &mut shape_state.blend_mode,
                                blend_mode,
                                format!("{blend_mode:?}"),
                            );
                        }
                    });
            });
            ui.end_row();
        });
}

fn shader_editor(ui: &mut egui::Ui, templates: &Templates, shape_state: &mut ShapeState) -> bool {
    let mut compile_shader = false;

    ui.horizontal(|ui| {
        for shader in [ShaderKind::Sdf, ShaderKind::Fill] {
            ui.selectable_value(
                &mut shape_state.selected_shader,
                shader,
                format!("{shader}"),
            );
        }

        ui.separator();

        if ui.button("Compile").clicked() {
            compile_shader = true;
        }

        ui.label("or press ctrl+enter");
        let ctrl_return = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Enter);
        if ui.input_mut(|i| i.consume_shortcut(&ctrl_return)) {
            compile_shader = true;
        }

        ui.separator();

        let template_button = ui.button("Template");
        egui::Popup::menu(&template_button)
            .id(egui::Id::new(format!(
                "template_menu_{}",
                shape_state.selected_shader
            ))) // Each popup should have its own state
            .align(egui::RectAlign::BOTTOM_START)
            .gap(4.0)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
            .show(|ui| {
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for template in templates.all_templates(shape_state.selected_shader) {
                            if ui.button(&template.name).clicked() {
                                let code = match shape_state.selected_shader {
                                    ShaderKind::Sdf => &mut shape_state.sdf_code,
                                    ShaderKind::Fill => &mut shape_state.fill_code,
                                };
                                code.clear();
                                code.push_str(&template.code);
                                compile_shader = true;
                            }
                        }
                    })
            });
    });

    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());

    let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
        let mut layout_job = egui_extras::syntax_highlighting::highlight(
            ui.ctx(),
            ui.style(),
            &theme,
            buf.as_str(),
            "rs", // There is no highlighter for wgsl yet
        );
        layout_job.wrap.max_width = wrap_width;
        ui.fonts_mut(|f| f.layout_job(layout_job))
    };

    let code = match shape_state.selected_shader {
        ShaderKind::Sdf => &mut shape_state.sdf_code,
        ShaderKind::Fill => &mut shape_state.fill_code,
    };
    egui::Frame::new()
        .inner_margin(egui::vec2(0.0, 4.0))
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("scroll_editor")
                .show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(code)
                            .id(egui::Id::new("editor"))
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .lock_focus(true)
                            .layouter(&mut layouter),
                    );
                });
        });

    compile_shader
}
