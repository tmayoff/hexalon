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

        let mut children = vec![];

        for x in -(size / 2)..(size / 2) {
            grid.cells.push(Vec::new());
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
                    HexCoord {
                        q: x + (size / 2),
                        r: y + (size / 2),
                    },
                    commands,
                    meshes,
                    materials,
                );

                grid.cells[(x + (size / 2)) as usize].push(id);
                children.push(id);
            }
        }

        commands.spawn(grid);
    }

    pub fn get_cells_in_line(&self, start: &HexCoord, end: &HexCoord) -> Vec<Entity> {
        let mut cells = Vec::new();

        let dx = (end.q - start.q).abs();
        let dy = (end.r - start.r).abs();
        let sx = if start.q < end.q { 1 } else { -1 };
        let sy = if start.r < end.r { 1 } else { -1 };
        let mut err = dx - dy;

        let mut q = start.q;
        let mut r = start.r;

        while q != end.q || r != end.r {
            cells.push(self.cells[q as usize][r as usize]);
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                q += sx;
            }
            if e2 < dx {
                err += dx;
                r += sy;
            }
        }

        cells.push(self.cells[end.q as usize][end.r as usize]); // Ensure the end point is included

        cells
    }

    fn world_pos_to_arr_pos(&self, pos: [i32; 2]) -> [i32; 2] {
        [pos[0] + (self.size / 2), pos[1] + (self.size / 2)]
    }
}
