use std::{cmp::max, collections::HashMap};

use crate::{
    cell::Cell,
    hex::{FractionalHexCoord, HexCoord},
};

use bevy::{prelude::*, sprite::ColorMaterial};

const HEX_SIZE: f32 = 35.0;
const HEX_SPACING: f32 = 1.0;

lazy_static! {
    static ref HEX_GRID_HORIZONTAL_OFFSET: f32 = 3_f32.sqrt();
}

#[derive(Event)]
pub enum GridEvent {
    Resize(i32, i32),
}

// TODO replace with normal matrices
#[derive(Default)]
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

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridEvent>()
            .add_systems(Update, on_grid_event);
    }
}

#[derive(Component, Default)]
pub struct Grid {
    pub size: i32,
    pub cells: HashMap<HexCoord, Entity>,

    orientation: Orientation,
    // forward: Mat4,
}

impl Grid {
    // Returns the left, right, top, bottom, edges for a grid of a certain size
    fn get_edges(size: i32) -> (i32, i32, i32, i32) {
        (-size / 2, size / 2, -size / 2, size / 2)
    }

    pub fn create(
        size: i32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let mut grid = Grid {
            size,
            cells: HashMap::new(),

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

    pub fn recreate(
        &mut self,
        size: i32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let old_size = self.size;
        let new_size = size;
        self.size = new_size;

        if new_size < old_size {
            let (left, right, top, bottom) = Grid::get_edges(old_size);
            for r in top..=bottom {
                let offset_r = (r as f32 / 2.0).floor() as i32;

                for q in (left - offset_r)..=(right - offset_r) {
                    let coord = HexCoord { q, r };
                    let e = self.cells.get(&coord);
                    match e {
                        Some(e) => {
                            commands.entity(*e).despawn_recursive();

                            self.cells.remove(&coord);
                        }
                        None => todo!(),
                    }
                }
            }
        }

        let (left, right, top, bottom) = Grid::get_edges(new_size);
        for r in top..=bottom {
            let offset_r = (r as f32 / 2.0).floor() as i32;

            for q in (left - offset_r)..=(right - offset_r) {
                let coord = HexCoord { q, r };

                let id = Cell::create(
                    self.hex_coord_to_pos(&coord),
                    HEX_SIZE,
                    HexCoord { q, r },
                    commands,
                    meshes,
                    materials,
                );

                self.cells.insert(HexCoord { q, r }, id);
            }
        }
    }

    pub fn get_cell(&self, pos: &HexCoord) -> Option<&Entity> {
        self.cells.get(pos)
    }

    pub fn get_neighbours(&self, pos: &HexCoord) -> Vec<HexCoord> {
        let directions = [
            HexCoord { q: 1, r: 0 },
            HexCoord { q: 1, r: -1 },
            HexCoord { q: 0, r: -1 },
            HexCoord { q: -1, r: 0 },
            HexCoord { q: -1, r: 1 },
            HexCoord { q: 0, r: 1 },
        ];

        let mut neighbours = Vec::new();

        directions.iter().for_each(|d| {
            if self.cells.contains_key(&(pos + d)) {
                neighbours.push(pos + d);
            }
        });

        neighbours
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
            if let Some(cell) = self.cells.get(&lerped) {
                cells.push(*cell);
            }
        }

        if let Some(cell) = self.cells.get(end) {
            cells.push(*cell);
        }

        cells
    }
}

fn on_grid_event(
    mut events: EventReader<GridEvent>,
    mut grid_q: Query<&mut Grid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut grid = grid_q.single_mut();

    for e in events.iter() {
        match e {
            GridEvent::Resize(q, r) => {
                grid.recreate(*q, &mut commands, &mut meshes, &mut materials);
            }
        }
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
