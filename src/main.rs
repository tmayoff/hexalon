#[macro_use]
extern crate lazy_static;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

lazy_static! {
    static ref HEX_GRID_HORIZONTAL_OFFSET: f32 = 3_f32.sqrt();
    static ref BACKGROUND_COLOR: Color = Color::WHITE;
    static ref HEX_OUTLINE_COLOR: Color = Color::GRAY;
    static ref CLEAR_COLOR: Color = *HEX_OUTLINE_COLOR;
}

fn main() {
    App::new()
        .insert_resource(ClearColor(*CLEAR_COLOR))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, background))
        .add_systems(Update, camera_controls)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const RADIUS: f32 = 50.0;
    const SPACING: f32 = 10.0;
    let horizontal_spacing: f32 = (*HEX_GRID_HORIZONTAL_OFFSET * RADIUS) + SPACING;
    let vertical_spacing: f32 = (3.0 / 2.0) * RADIUS + SPACING;

    const GRID_SIZE: i32 = 10;

    for x in -(GRID_SIZE / 2)..(GRID_SIZE / 2) {
        for y in -(GRID_SIZE / 2)..(GRID_SIZE / 2) {
            let x_offset = if y % 2 != 0 {
                RADIUS - SPACING * 2.0
            } else {
                0.0
            };

            let color = if y % 2 == 0 {
                *BACKGROUND_COLOR
            } else {
                Color::rgb(1.0, 0.0, 0.0)
            };

            let x_pos = x as f32 * horizontal_spacing + x_offset;
            let y_pos = y as f32 * vertical_spacing;

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::RegularPolygon::new(RADIUS, 6)))
                    .into(),
                material: materials.add(color.into()),
                transform: Transform::from_translation(Vec3::new(x_pos, y_pos, 0.0)),
                ..Default::default()
            });
        }
    }
}

fn camera_controls() {}
