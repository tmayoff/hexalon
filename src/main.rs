#[macro_use]
extern crate lazy_static;

mod cell;
mod draw;
mod grid;
mod hex;
mod initiative_tracker;
mod token;
mod ui;

use bevy::{
    audio::AudioPlugin,
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_mod_reqwest::ReqwestPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};

use draw::Draw;
use grid::Grid;

use crate::initiative_tracker::Tracker;

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
            ReqwestPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                initiative_tracker::send_request,
                initiative_tracker::handle_response,
                ui::gui,
                draw::on_draw,
                token::on_token_event,
                token::on_tracker_event,
            ),
        )
        .add_event::<cell::CellEvent>()
        .add_event::<token::TokenEvent>()
        .add_event::<initiative_tracker::TrackerEvent>()
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_millis(500),
            TimerMode::Repeating,
        )))
        .run();
}

#[derive(Resource)]
struct ReqTimer(pub Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Setup Grid
    const GRID_SIZE: i32 = 10;
    Grid::create(GRID_SIZE, &mut commands, &mut meshes, &mut materials);

    commands.spawn(Draw::default());
    commands.spawn(Tracker::default());

    // Setup Camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
        PanCam {
            grab_buttons: vec![MouseButton::Middle],
            ..Default::default()
        },
        RaycastPickCamera::default(),
    ));
}
