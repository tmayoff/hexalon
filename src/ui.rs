use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::draw::{Draw, DrawMode};
use crate::token::{TokenEvent, TokenType};

pub fn gui(
    mut contexts: EguiContexts,
    mut draw_q: Query<&mut Draw>,
    mut token_event: EventWriter<TokenEvent>,
    cam_q: Query<&Transform, With<Camera2d>>,
) {
    let mut draw = draw_q.single_mut();

    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button(
                "File",
                |ui| {
                    if ui.button("Open Obsidian project").clicked() {}
                },
            );
        });
    });

    egui::Window::new("Toolbox").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.heading("Drawing");
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
                    let mut color = draw.color.as_rgba_u8();
                    ui.color_edit_button_srgba_unmultiplied(&mut color);
                    draw.color = Color::from(color.map(|c| c as f32 / 255.0));
                    ui.end_row();
                });

            ui.heading("Tokens");
            if ui.button("Spawn Enemy").clicked() {
                let cam = cam_q.single();

                token_event.send(TokenEvent::Spawn((TokenType::Enemy, *cam)))
            }

            if ui.button("Spawn Party Member").clicked() {
                let cam = cam_q.single();
                token_event.send(TokenEvent::Spawn((TokenType::Party, *cam)))
            }
        });
    });
}
