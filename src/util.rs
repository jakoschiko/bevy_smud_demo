use bevy::prelude::*;
use bevy_egui::egui;

pub fn convert_color(color: egui::Color32) -> Color {
    let [r, g, b, a] = color.to_array();
    Color::srgba_u8(r, g, b, a)
}
