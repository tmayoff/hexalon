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
use bevy_mod_reqwest::{reqwest, ReqwestBytesResult, ReqwestPlugin, ReqwestRequest};
use bevy_pancam::{PanCam, PanCamPlugin};

use draw::Draw;
use grid::Grid;
use token::TurnEvent;

use crate::initiative_tracker::{Data, Tracker};

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
                send_request,
                handle_response,
                ui::gui,
                draw::on_draw,
                token::on_token_event,
                token::on_turn_update,
            ),
        )
        .add_event::<cell::CellEvent>()
        .add_event::<token::TokenEvent>()
        .add_event::<token::TurnEvent>()
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
    commands.spawn(Tracker { data: None });

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

fn send_request(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ReqTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        let req = reqwest::Request::new(
            reqwest::Method::GET,
            "http://127.0.0.1:8080/ttrpg_data".try_into().unwrap(),
        );

        commands.spawn(ReqwestRequest::new(req));
    }
}

fn handle_response(
    mut commands: Commands,
    mut event_writer: EventWriter<TurnEvent>,
    results: Query<(Entity, &ReqwestBytesResult)>,
    mut tracker_q: Query<&mut Tracker>,
) {
    let mut tracker = tracker_q.single_mut();

    for (e, res) in results.iter() {
        match &res.0 {
            Ok(_) => {
                let old_data = &tracker.data;
                let new_data = res
                    .deserialize_json::<Data>()
                    .expect("Failed to deserialize data");

                match old_data {
                    Some(old_data) => {
                        if old_data.state != new_data.state {
                            event_writer.send(TurnEvent);
                        }

                        tracker.data = Some(new_data);
                    }
                    None => {
                        tracker.data = Some(new_data);
                    }
                }
            }
            Err(e) => log::error!("{:?}", e),
        }
        commands.entity(e).despawn_recursive();
    }
}
