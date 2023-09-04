use crate::cell::Cell;

use bevy::{prelude::*, sprite::ColorMaterial};
use bevy_mod_picking::prelude::*;

const HEX_SIZE: f32 = 35.0;
const HEX_SPACING: f32 = 2.0;

lazy_static! {
    pub static ref HEX_HOVER_COLOR: Color = Color::Rgba {
        red: 0.9,
        green: 0.9,
        blue: 0.9,
        alpha: 1.0
    };
    pub static ref HEX_PRESSED_COLOR: Color = Color::Rgba {
        red: 0.5,
        green: 0.5,
        blue: 0.5,
        alpha: 1.0
    };
    static ref HEX_GRID_HORIZONTAL_OFFSET: f32 = 3_f32.sqrt();
}

struct Selection {
    start: Entity,
    end: Option<Entity>,
}

#[derive(Component)]
pub struct Grid {
    pub size: i32,
    pub grid: Vec<Vec<Entity>>,

    selection: Option<Selection>,
}

impl Grid {
    pub fn create(
        size: i32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let horizontal_spacing: f32 = (*HEX_GRID_HORIZONTAL_OFFSET * HEX_SIZE) + HEX_SPACING;
        let vertical_spacing: f32 = 1.5 * HEX_SIZE + HEX_SPACING;

        let mut grid = Grid {
            size,
            grid: Vec::new(),
            selection: None,
        };

        let mut children = vec![];

        for x in -(size / 2)..(size / 2) {
            grid.grid.push(Vec::new());
            for y in -(size / 2)..(size / 2) {
                let x_offset = if y % 2 == 0 {
                    horizontal_spacing / 2.0
                } else {
                    0.0
                };

                let x_pos = x as f32 * horizontal_spacing + x_offset;
                let y_pos = y as f32 * vertical_spacing;

                let id = Cell::create(
                    Vec2::new(x_pos, y_pos),
                    HEX_SIZE,
                    commands,
                    meshes,
                    materials,
                );

                grid.grid[(x + (size / 2)) as usize].push(id);
                children.push(id);
            }
        }

        commands.spawn(grid);
    }
}

pub fn grid_selection_down(
    event: Listener<Pointer<Down>>,
    mut grid_q: Query<&mut Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_q: Query<(&mut Cell, &Handle<ColorMaterial>)>,
) {
    let mut grid = grid_q.single_mut();

    grid.selection = Some(Selection {
        start: event.target,
        end: None,
    });

    let (mut cell, mat) = cell_q.get_mut(event.target).unwrap();
    cell.color_managed = true;

    let mat = materials.get_mut(mat).unwrap();
    mat.color = Color::BLUE;
}

pub fn grid_selection_up(
    event: Listener<Pointer<Up>>,
    mut grid_q: Query<&mut Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_q: Query<(&mut Cell, &Handle<ColorMaterial>)>,
) {
    let grid = grid_q.single_mut();
    if grid.selection.is_some() {
        // let (_, mat) = cell_q.get(selection.start).unwrap();
        // let mat = materials.get_mut(mat).unwrap();
        // mat.color = Color::BLUE;

        // Selection must have started
        let (mut cell, mat) = cell_q.get_mut(event.target).unwrap();
        cell.color_managed = true;

        let mat = materials.get_mut(mat).unwrap();
        mat.color = Color::BLUE;
    }
}
