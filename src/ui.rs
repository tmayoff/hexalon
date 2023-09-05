use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::draw::{Draw, DrawMode};

pub fn gui(mut contexts: EguiContexts, mut draw_q: Query<&mut Draw>) {
    let mut draw = draw_q.single_mut();

    let ctx = contexts.ctx_mut();

    egui::Window::new("Draw Settings").show(ctx, |ui| {
        egui::Grid::new("draw_settings")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Mode");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut draw.draw_mode, DrawMode::Cell, "Cell");
                    ui.radio_value(&mut draw.draw_mode, DrawMode::Box, "Box");
                    ui.radio_value(&mut draw.draw_mode, DrawMode::Line, "Line");
                });
                ui.end_row();

                ui.label("Color");
                let mut color = draw.color.as_rgba_f32();
                ui.color_edit_button_rgba_unmultiplied(&mut color);
                draw.color = color.into();
                ui.end_row();
            });
    });
}
