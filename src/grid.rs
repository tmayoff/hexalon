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
    pub cells: Vec<Vec<Entity>>,
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
            cells: Vec::new(),
        };

        let mut temp_cells = vec![vec![None; size as usize]; size as usize];

        for r in 0..size {
            for q in 0..size {
                let x_offset = if r % 2 == 0 {
                    horizontal_spacing / 2.0
                } else {
                    0.0
                };

                let x_pos = (q - (size / 2)) as f32 * horizontal_spacing + x_offset;
                let y_pos = (r - (size / 2)) as f32 * vertical_spacing;

                let id = Cell::create(
                    Vec2::new(x_pos, y_pos),
                    HEX_SIZE,
                    HexCoord { q, r },
                    commands,
                    meshes,
                    materials,
                );

                let index_q = q as f32 + (r as f32 / 2.0).floor();
                temp_cells[r as usize][(index_q) as usize] = Some(id);
            }
        }

        for row in temp_cells {
            let mut row_vec = Vec::new();
            for cell in row {
                row_vec.push(cell.unwrap());
            }
            grid.cells.push(row_vec);
        }

        commands.spawn(grid);
    }

    pub fn get_cells_in_line(&self, start: &HexCoord, end: &HexCoord) -> Vec<Entity> {
        let mut cells = Vec::new();

        let n = Self::axial_distance(start, end);
        for i in 0..n {
            let lerped = Self::cube_lerp(start, end, 1.0 / n as f32 * i as f32);
            let rounded = Self::cube_round(lerped.q as f32, lerped.r as f32);
            cells.push(self.cells[rounded.q as usize][rounded.r as usize]);
        }

        cells
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    fn cube_lerp(a: &HexCoord, b: &HexCoord, t: f32) -> HexCoord {
        HexCoord {
            q: Grid::lerp(a.q as f32, b.q as f32, t).round() as i32,
            r: Grid::lerp(a.r as f32, b.r as f32, t).round() as i32,
        }
    }

    fn axial_subtract(a: &HexCoord, b: &HexCoord) -> HexCoord {
        HexCoord {
            q: a.q - b.q,
            r: a.r - b.r,
        }
    }

    fn axial_distance(a: &HexCoord, b: &HexCoord) -> i32 {
        let vec = Self::axial_subtract(a, b);
        (vec.q.abs() + vec.r.abs() + (vec.q + vec.r).abs()) / 2
    }

    fn cube_round(frac_q: f32, frac_r: f32) -> HexCoord {
        let mut q = (frac_q).round();
        let mut r = (frac_r).round();
        let mut s = (-frac_q - frac_r).round();

        let q_diff = (q - frac_q).abs();
        let r_diff = (r - frac_r).abs();
        let s_diff = (s - (-frac_q - frac_r)).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s
        } else {
            s = -q - r
        }

        HexCoord {
            q: q as i32,
            r: r as i32,
        }
    }
}
