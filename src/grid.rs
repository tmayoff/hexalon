use crate::cell::Cell;

use bevy::{prelude::*, sprite::ColorMaterial};

const HEX_SIZE: f32 = 35.0;
const HEX_SPACING: f32 = 2.0;

lazy_static! {
    static ref HEX_TINT_COLOR: Color = Color::GRAY;
    static ref HEX_GRID_HORIZONTAL_OFFSET: f32 = 3_f32.sqrt();
    static ref HEX_COLOR: Color = Color::BLUE;
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
