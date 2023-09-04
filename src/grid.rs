use crate::cell::Cell;

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
    static ref HEX_COLOR: Color = Color::Rgba {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0
    };
}

#[derive(Component)]
pub struct Grid {
    pub size: i32,
    pub grid: Vec<Vec<Entity>>,
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
        };

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
                    *HEX_COLOR,
                    commands,
                    meshes,
                    materials,
                );

                grid.grid[(x + (size / 2)) as usize].push(id);
            }
        }

        commands.spawn(grid);
    }
}
