use bevy::prelude::*;

use crate::{
    cell::{self, Cell, CellEvent},
    grid::{self, Grid},
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

#[derive(Component)]
pub struct Draw {
    pub draw_mode: DrawMode,
    pub color: Color,

    start_cell: Option<Entity>,

    last_hint: Vec<Entity>,
}

impl Default for Draw {
    fn default() -> Self {
        Self {
            draw_mode: DrawMode::Cell,
            color: Color::WHITE,
            start_cell: None,
            last_hint: Vec::new(),
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
                    draw_cell(cell, draw.color, &mut cell_q, &mut materials);
                }
            }
            CellEvent::Released(cell) => {
                if let Some(start_cell) = draw.start_cell {
                    draw_line(
                        &start_cell,
                        cell,
                        draw.color,
                        &mut cell_q,
                        &mut materials,
                        &mut draw,
                        grid,
                    );
                }
                draw.start_cell = None;
                draw.last_hint = Vec::new();
            }
            CellEvent::Over(cell) => match draw.draw_mode {
                DrawMode::Cell => {
                    if draw.start_cell.is_some() {
                        draw_cell(cell, draw.color, &mut cell_q, &mut materials)
                    }
                }
                DrawMode::Box => todo!("Draw Hints"),
                DrawMode::Line => {
                    if let Some(start_cell) = draw.start_cell {
                        draw_line(
                            &start_cell,
                            cell,
                            *grid::HEX_HOVER_COLOR,
                            &mut cell_q,
                            &mut materials,
                            &mut draw,
                            grid,
                        );
                    }
                }
            },
        }
    }
}

fn draw_cell(
    cell: &Entity,
    color: Color,
    cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let (mut cell, mat) = cell_q.get_mut(*cell).unwrap();
    let mat = materials.get_mut(mat).unwrap();
    cell.color_managed = true;
    mat.color = color;
}

fn draw_line(
    start: &Entity,
    end: &Entity,
    color: Color,
    cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    draw: &mut Draw,
    grid: &Grid,
) {
    let start = cell_q.get(*start).unwrap().0;
    let end = cell_q.get(*end).unwrap().0;

    let cells = grid.get_cells_in_line(&start.pos, &end.pos);

    for cell in &draw.last_hint {
        draw_cell(cell, *cell::HEX_COLOR, cell_q, materials);
    }

    for cell in &cells {
        draw_cell(cell, color, cell_q, materials);
    }

    draw.last_hint = cells;
}

fn draw_box() {
    todo!();
}
