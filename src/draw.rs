use std::cmp;

use bevy::{math::vec4, prelude::*};

use crate::{
    cell::{Cell, CellEvent},
    grid::Grid,
    hex::HexCoord,
};

#[derive(PartialEq, Eq)]
pub enum DrawMode {
    Cell,
    Box,
    Line,
}

#[derive(Clone, Copy)]
pub struct Selection {
    pub start: Entity,
    pub end: Option<Entity>,
}

enum DrawColor {
    Color(Color),
    Hint,
}

#[derive(Component)]
pub struct Draw {
    pub draw_mode: DrawMode,
    pub fill: bool,
    pub color: Color,

    start_cell: Option<Entity>,

    last_hint: Vec<Entity>,
}

impl Default for Draw {
    fn default() -> Self {
        Self {
            draw_mode: DrawMode::Cell,
            fill: true,
            color: Color::BLUE,
            start_cell: None,
            last_hint: Vec::new(),
        }
    }
}

impl Draw {
    fn reset_hints(
        &mut self,
        cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        for cell in &self.last_hint {
            let (c, _) = cell_q.get(*cell).unwrap();
            Self::draw_cell_color(cell, DrawColor::Color(c.color), cell_q, materials);
        }

        self.last_hint.clear();
    }

    fn draw_box(
        &mut self,
        start: &Entity,
        end: &Entity,
        cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        grid: &Grid,
        hint: bool,
    ) {
        let (start_cell, _) = cell_q.get(*start).unwrap();
        let (end_cell, _) = cell_q.get(*end).unwrap();
        let start_pos = start_cell.pos;
        let end_pos = end_cell.pos;

        let cells = grid.get_cells_in_box(&start_pos, &end_pos);

        for cell in &cells {
            self.draw_cell(cell, cell_q, materials, hint);
        }

        if self.fill {
            let start_q = cmp::min(start_pos.q, end_pos.q);
            let end_q = cmp::max(start_pos.q, end_pos.q);
            let start_r = cmp::min(start_pos.r, end_pos.r);
            let end_r = cmp::max(start_pos.r, end_pos.r);

            for q in start_q..end_q {
                for r in start_r..end_r {
                    let pos = HexCoord { q, r };
                    let c = grid.get_cell(&pos);
                    if let Some(cell) = c {
                        self.draw_cell(cell, cell_q, materials, hint);
                    }
                }
            }
        }

        self.last_hint = cells;
    }

    fn draw_line(
        &mut self,
        start: &Entity,
        end: &Entity,
        cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        grid: &Grid,
        hint: bool,
    ) {
        let start = cell_q.get(*start).unwrap().0;
        let end = cell_q.get(*end).unwrap().0;

        let cells = grid.get_cells_in_line(&start.pos, &end.pos);

        for cell in &cells {
            self.draw_cell(cell, cell_q, materials, hint);
        }

        self.last_hint = cells;
    }

    fn draw_cell(
        &self,
        cell: &Entity,
        cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        hint: bool,
    ) {
        Self::draw_cell_color(
            cell,
            if hint {
                DrawColor::Hint
            } else {
                DrawColor::Color(self.color)
            },
            cell_q,
            materials,
        );
    }

    fn draw_cell_color(
        cell: &Entity,
        color: DrawColor,
        cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let (mut cell, mat) = cell_q.get_mut(*cell).unwrap();
        let mat = materials.get_mut(mat).unwrap();

        match color {
            DrawColor::Color(c) => {
                mat.color = c;
                cell.color = c;
            }
            DrawColor::Hint => mat.color = cell.color + vec4(-0.2, -0.2, -0.2, 0.0),
        }
    }
}

pub fn on_draw(
    mut draw_event: EventReader<CellEvent>,
    mut draw_q: Query<&mut Draw>,
    mut cell_q: Query<(&mut Cell, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_q: Query<&Grid>,
) {
    let mut draw = draw_q.single_mut();
    let grid = grid_q.single();

    for event in draw_event.iter() {
        match event {
            CellEvent::Pressed(cell) => {
                draw.start_cell = Some(*cell);

                if draw.draw_mode == DrawMode::Cell {
                    draw.draw_cell(cell, &mut cell_q, &mut materials, false);
                }
            }
            CellEvent::Released(distance) => {
                // Get end cell
                draw.reset_hints(&mut cell_q, &mut materials);
                if let Some(start_entity) = draw.start_cell {
                    let (start_cell, _) = cell_q.get(start_entity).unwrap();
                    let c = grid.hex_coord_to_pos(&start_cell.pos) + *distance;

                    let end_cell = grid.get_cell(&grid.pos_to_hex_coord(&c));
                    if let Some(end_cell) = end_cell {
                        if draw.draw_mode == DrawMode::Line {
                            draw.draw_line(
                                &start_entity,
                                end_cell,
                                &mut cell_q,
                                &mut materials,
                                grid,
                                false,
                            );
                            draw.last_hint = Vec::new();
                        } else if draw.draw_mode == DrawMode::Box {
                            draw.draw_box(
                                &start_entity,
                                end_cell,
                                &mut cell_q,
                                &mut materials,
                                grid,
                                false,
                            );
                        }
                    }
                }

                draw.start_cell = None;
            }
            CellEvent::Over(cell) => match draw.draw_mode {
                DrawMode::Cell => {
                    if draw.start_cell.is_some() {
                        draw.draw_cell(cell, &mut cell_q, &mut materials, false)
                    }
                }
                DrawMode::Box => {
                    // Draw hints
                    draw.reset_hints(&mut cell_q, &mut materials);
                    if let Some(start_cell) = draw.start_cell {
                        draw.draw_box(&start_cell, cell, &mut cell_q, &mut materials, grid, true);
                    }
                }
                DrawMode::Line => {
                    // Draw hints
                    draw.reset_hints(&mut cell_q, &mut materials);
                    if let Some(start_cell) = draw.start_cell {
                        draw.draw_line(&start_cell, cell, &mut cell_q, &mut materials, grid, true);
                    }
                }
            },
        }
    }
}
