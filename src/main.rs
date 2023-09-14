#[macro_use]
extern crate lazy_static;

mod cell;
mod draw;
mod grid;
mod hex;
mod token;
mod ui;

use bevy::{audio::AudioPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

use draw::Draw;
use grid::Grid;

lazy_static! {
    static ref HEX_OUTLINE_COLOR: Color = Color::Rgba {
        red: 0.25,
        green: 0.25,
        blue: 0.25,
        alpha: 1.0
    };
    static ref CLEAR_COLOR: Color = *HEX_OUTLINE_COLOR;
}

fn main() {
    App::new()
        .insert_resource(ClearColor(*CLEAR_COLOR))
        .add_plugins((
            DefaultPlugins.build().disable::<AudioPlugin>(),
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>()
                .disable::<DefaultHighlightingPlugin>(),
            PanCamPlugin,
            EguiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (ui::gui, draw::on_draw, token::on_token_event))
        .add_event::<cell::CellEvent>()
        .add_event::<token::TokenEvent>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Setup Grid
    const GRID_SIZE: i32 = 10;
    Grid::create(GRID_SIZE, &mut commands, &mut meshes, &mut materials);

    commands.spawn(Draw::default());

    // Setup Camera
    commands.spawn((
        Camera2dBundle::default(),
        PanCam {
            grab_buttons: vec![MouseButton::Middle],
            ..Default::default()
        },
        RaycastPickCamera::default(),
    ));
}
