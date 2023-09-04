#[macro_use]
extern crate lazy_static;

mod cell;
mod grid;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

use grid::Grid;

lazy_static! {
    static ref HEX_OUTLINE_COLOR: Color = Color::GRAY;
    static ref CLEAR_COLOR: Color = *HEX_OUTLINE_COLOR;
}

fn main() {
    App::new()
        .insert_resource(ClearColor(*CLEAR_COLOR))
        .add_plugins((
            DefaultPickingPlugins,
            DefaultPlugins.set(low_latency_window_plugin()),
            PanCamPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Setup Camera
    commands.spawn((Camera2dBundle::default(), PanCam::default()));

    // Setup Grid
    const GRID_SIZE: i32 = 250;
    Grid::create(GRID_SIZE, &mut commands, &mut meshes, &mut materials);
}
