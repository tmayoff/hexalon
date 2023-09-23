use bevy::prelude::*;

use crate::{
    cell::{Cell, CellEvent},
    grid::{Grid, HEX_HOVER_COLOR},
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
            self.draw_cell_color(cell, c.color, cell_q, materials, false);
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
        let start = cell_q.get(*start).unwrap();
        let end = cell_q.get(*end).unwrap();

        let cells = grid.get_cells_in_box(&start.0.pos, &end.0.pos);

        for cell in &cells {
            self.draw_cell(cell, cell_q, materials, hint);
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
        let color = if hint { *HEX_HOVER_COLOR } else { self.color };
        self.draw_cell_color(cell, color, cell_q, materials, hint);
    }

    fn draw_cell_color(
        &self,
        cell: &Entity,
        color: Color,
        cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        hint: bool,
    ) {
        let (mut cell, mat) = cell_q.get_mut(*cell).unwrap();
        let mat = materials.get_mut(mat).unwrap();

        mat.color = color;

        if !hint {
            cell.color = color;
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
            CellEvent::Released(cell) => {
                if draw.draw_mode == DrawMode::Line {
                    if let Some(start_cell) = draw.start_cell {
                        draw.draw_line(
                            &start_cell,
                            &cell.unwrap(),
                            &mut cell_q,
                            &mut materials,
                            grid,
                            false,
                        );
                    }
                    draw.last_hint = Vec::new();
                } else if draw.draw_mode == DrawMode::Box {
                    if let Some(start_cell) = draw.start_cell {
                        draw.draw_box(
                            &start_cell,
                            &cell.unwrap(),
                            &mut cell_q,
                            &mut materials,
                            grid,
                            false,
                        );
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
