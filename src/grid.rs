use std::{cmp::max, collections::HashMap};

use crate::cell::{Cell, HexCoord};

use bevy::{prelude::*, sprite::ColorMaterial};

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

#[derive(Component)]
pub struct Grid {
    pub size: i32,
    pub cells: HashMap<HexCoord, Entity>,
}

impl Grid {
    pub fn create(
        size: i32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let mut grid = Grid {
            size,
            cells: HashMap::new(),
        };

        let left: i32 = -size / 2;
        let right: i32 = size / 2;
        let top: i32 = -size / 2;
        let bottom: i32 = size / 2;

        let f0: f32 = 3.0_f32.sqrt();
        let f1: f32 = 3.0_f32.sqrt() / 2.0;
        let f2: f32 = 0.0;
        let f3: f32 = 3.0 / 2.0;

        for r in top..=bottom {
            let offset_r = (r as f32 / 2.0).floor() as i32;

            for q in (left - offset_r)..=(right - offset_r) {
                let x_pos = (f0 * q as f32 + f1 * r as f32) * (HEX_SIZE + HEX_SPACING);
                let y_pos = (f2 * q as f32 + f3 * r as f32) * (HEX_SIZE + HEX_SPACING);

                let id = Cell::create(
                    Vec2::new(x_pos, y_pos),
                    HEX_SIZE,
                    HexCoord { q, r },
                    commands,
                    meshes,
                    materials,
                );

                grid.cells.insert(HexCoord { q, r }, id);
            }
        }

        commands.spawn(grid);
    }

    pub fn get_cells_in_box(&self, start: &HexCoord, end: &HexCoord) -> Vec<Entity> {
        let mut cells = vec![
            *self.cells.get(start).unwrap(),
            *self.cells.get(end).unwrap(),
        ];

        let c1 = HexCoord {
            q: start.q,
            r: end.r,
        };

        let c2 = HexCoord {
            q: end.q,
            r: start.r,
        };

        cells.append(&mut self.get_cells_in_line(start, &c1));
        cells.append(&mut self.get_cells_in_line(start, &c2));
        cells.append(&mut self.get_cells_in_line(&c1, end));
        cells.append(&mut self.get_cells_in_line(&c2, end));

        cells
    }

    pub fn get_cells_in_line(&self, start: &HexCoord, end: &HexCoord) -> Vec<Entity> {
        let mut cells = Vec::new();

        let n = Self::axial_distance(start, end);
        let step = 1.0 / max(n, 1) as f32;
        for i in 0..n {
            let lerped = HexCoord::lerp(start, end, step * i as f32);
            cells.push(self.cells.get(&lerped).unwrap().to_owned());
        }

        cells.push(self.cells.get(end).unwrap().to_owned());

        cells
    }

    fn axial_distance(a: &HexCoord, b: &HexCoord) -> i32 {
        let vec = a - b;
        (vec.q.abs() + vec.r.abs() + (vec.q + vec.r).abs()) / 2
    }
}
