use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::draw::{Draw, DrawMode};
use crate::grid::Grid;
use crate::initiative_tracker::Tracker;
use crate::token::{Token, TokenEvent, TokenType};

pub fn gui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut draw_q: Query<&mut Draw>,
    mut token_event: EventWriter<TokenEvent>,
    grid_q: Query<&Grid>,
    cam_q: Query<&Transform, With<Camera2d>>,
    token_q: Query<Entity, With<Token>>,
    tracker_q: Query<&Tracker>,
) {
    let mut draw = draw_q.single_mut();
    let tracker = tracker_q.single();
    let grid = grid_q.single();

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

            if token_q.is_empty() {
                if let Some(tracker_data) = &tracker.data {
                    if ui.button("Load State").clicked() {
                        let cam = cam_q.single();
                        let pos = Vec2 {
                            x: cam.translation.x,
                            y: cam.translation.z,
                        };
                        let coords = grid.pos_to_hex_coord(&pos);

                        let batches = tracker_data
                            .state
                            .creatures
                            .iter()
                            .map(|c| match c.player {
                                Some(_) => (
                                    Token::new(
                                        &c.id,
                                        &c.name,
                                        TokenType::Party,
                                        &coords,
                                        &Color::BLUE,
                                    ),
                                    pos,
                                ),
                                None => (
                                    Token::new(
                                        &c.id,
                                        &c.name,
                                        TokenType::Enemy,
                                        &coords,
                                        &Color::rgb(0.93, 0.13, 0.25),
                                    ),
                                    pos,
                                ),
                            })
                            .collect();
                        token_event.send(TokenEvent::BatchSpawn(batches))
                    }
                }
            } else if ui.button("Clear state").clicked() {
                token_q
                    .iter()
                    .for_each(|e| commands.entity(e).despawn_recursive())
            }
        });
    });
}
