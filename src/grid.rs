use std::{cmp::max, collections::HashMap};

use crate::{
    cell::Cell,
    hex::{FractionalHexCoord, HexCoord},
};

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

// TODO replace with normal matrices
struct Orientation {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,

    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,
}

#[derive(Component)]
pub struct Grid {
    pub size: i32,
    pub cells: HashMap<HexCoord, Entity>,

    orientation: Orientation,
    // forward: Mat4,
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
            tokens: HashMap::new(),

            orientation: Orientation {
                f0: 3.0_f32.sqrt(),
                f1: 3.0_f32.sqrt() / 2.0,
                f2: 0.0,
                f3: 3.0 / 2.0,
                b0: 3.0_f32.sqrt() / 3.0,
                b1: -1.0 / 3.0,
                b2: 0.0,
                b3: 2.0 / 3.0,
            },
        };

        let left: i32 = -size / 2;
        let right: i32 = size / 2;
        let top: i32 = -size / 2;
        let bottom: i32 = size / 2;

        for r in top..=bottom {
            let offset_r = (r as f32 / 2.0).floor() as i32;

            for q in (left - offset_r)..=(right - offset_r) {
                let id = Cell::create(
                    grid.hex_coord_to_pos(&HexCoord { q, r }),
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

    pub fn pos_to_hex_coord(&self, pos: &Vec2) -> HexCoord {
        let ori = &self.orientation;

        let pt = Vec2 {
            x: pos.x / (HEX_SIZE + HEX_SPACING),
            y: pos.y / (HEX_SIZE + HEX_SPACING),
        };

        let q = ori.b0 * pt.x + ori.b1 * pt.y;
        let r = ori.b2 * pt.x + ori.b3 * pt.y;

        HexCoord::round(FractionalHexCoord { q, r })
    }

    pub fn hex_coord_to_pos(&self, coord: &HexCoord) -> Vec2 {
        let ori = &self.orientation;

        let x = (ori.f0 * coord.q as f32 + ori.f1 * coord.r as f32) * (HEX_SIZE + HEX_SPACING);
        let y = (ori.f2 * coord.q as f32 + ori.f3 * coord.r as f32) * (HEX_SIZE + HEX_SPACING);

        Vec2 { x, y }
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

        let n = start.distance(end);
        let step = 1.0 / max(n, 1) as f32;
        for i in 0..n {
            let lerped = HexCoord::lerp(start, end, step * i as f32);
            cells.push(self.cells.get(&lerped).unwrap().to_owned());
        }

        cells.push(self.cells.get(end).unwrap().to_owned());

        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_pos() {
        let grid = Grid {
            size: 250,
            cells: HashMap::new(),
            tokens: HashMap::new(),

            orientation: Orientation {
                f0: 3.0_f32.sqrt(),
                f1: 3.0_f32.sqrt() / 2.0,
                f2: 0.0,
                f3: 3.0 / 2.0,
                b0: 3.0_f32.sqrt() / 3.0,
                b1: -1.0 / 3.0,
                b2: 0.0,
                b3: 2.0 / 3.0,
            },
        };

        let test_coord = HexCoord { q: 10, r: 10 };

        let pos = grid.hex_coord_to_pos(&test_coord);
        let coord = grid.pos_to_hex_coord(&pos);

        assert_eq!(test_coord, coord);
    }
}
