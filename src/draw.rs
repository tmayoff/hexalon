use bevy::prelude::*;

use crate::cell::Cell;

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
}

impl Default for Draw {
    fn default() -> Self {
        Self {
            draw_mode: DrawMode::Cell,
            color: Color::WHITE,
        }
    }
}

#[derive(Event)]
pub struct DrawEvent {
    pub selection: Selection,
}

pub fn draw(
    mut draw_event: EventReader<DrawEvent>,
    draw_q: Query<&Draw>,
    mut cell_q: Query<(&mut Cell, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let draw = draw_q.single();

    for event in draw_event.iter() {
        match draw.draw_mode {
            DrawMode::Cell => {
                if let Some(end) = event.selection.end {
                    draw_cell(end, draw.color, &mut cell_q, &mut materials);
                }
            }
            DrawMode::Box => draw_box(),
            DrawMode::Line => draw_line(),
        }
    }
}

fn draw_cell(
    cell: Entity,
    color: Color,
    cell_q: &mut Query<(&mut Cell, &Handle<ColorMaterial>)>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let (mut cell, mat) = cell_q.get_mut(cell).unwrap();
    let mat = materials.get_mut(mat).unwrap();
    cell.color_managed = true;
    mat.color = color;
}

fn draw_box() {
    todo!();
}

fn draw_line() {
    todo!();
}
