use std::fmt::Display;

use bevy::prelude::*;
use bevy_egui::egui;
use bevy_smud::BlendMode;

use crate::consts;

type ShaderId = u32;
type ShapeId = u32;

#[derive(Resource)]
pub struct GlobalState {
    pub camera_position: Vec2,
    pub background_color: egui::Color32,
    next_shader_id: ShapeId,
    next_shape_id: ShapeId,
    pub selected_tab: SelectedTab,
    scroll_to: Option<ShapeId>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            camera_position: Vec2::new(-consts::SIDE_PANEL_WIDTH / 2.0, 0.0),
            background_color: egui::Color32::from_rgb(43, 44, 47), // Same as default ClearColor
            next_shader_id: 0,
            next_shape_id: 0,
            selected_tab: SelectedTab::Global,
            scroll_to: None,
        }
    }
}

impl GlobalState {
    pub fn create_shader(&mut self) -> ShaderId {
        let id = self.next_shader_id;
        self.next_shader_id += 1;
        id
    }

    pub fn create_shape(&mut self) -> ShapeId {
        let id = self.next_shape_id;
        self.next_shape_id += 1;
        self.select_tab(SelectedTab::Shape(id));
        id
    }

    pub fn select_tab(&mut self, tab: SelectedTab) {
        self.selected_tab = tab;
        if let SelectedTab::Shape(id) = tab {
            self.scroll_to = Some(id);
        }
    }

    pub fn check_scroll_to(&mut self, shape: ShapeId) -> bool {
        if self.scroll_to == Some(shape) {
            self.scroll_to = None;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectedTab {
    Global,
    Shape(u32),
}

#[derive(Clone, Component)]
pub struct ShapeState {
    pub id: u32,
    pub position: Vec3,
    pub rotation: f32,
    pub scale: f32,
    pub color: egui::Color32,
    pub selected_shader: ShaderKind,
    pub sdf_code: String,
    pub fill_code: String,
    pub bounds_length: f32,
    pub params: Vec4,
    pub blend_mode: BlendMode,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ShaderKind {
    Sdf,
    Fill,
}

impl Display for ShaderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderKind::Sdf => write!(f, "sdf"),
            ShaderKind::Fill => write!(f, "fill"),
        }
    }
}
