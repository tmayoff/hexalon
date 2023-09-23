mod state;

use bevy::prelude::*;
use bevy_mod_reqwest::{reqwest, ReqwestBytesResult, ReqwestRequest};
use serde::Deserialize;

use crate::ReqTimer;
use state::State;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Player {
    ac: i32,
    hp: i32,
    level: i32,
    modifier: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Monster {
    display: Option<String>,
    ac: i32,
    hp: i32,
    cr: String,
    // currentAC: i32,
    // currentHP: i32,
    enabled: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Party {
    pub name: String,
    pub players: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum CreatureType {
    Player(Player),
    Monster(Monster),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Creature {
    pub id: String,
    pub name: String,
    pub initiative: i32,
    pub player: Option<bool>,
    pub active: bool,
    pub number: i32,
    pub cr: Option<String>,
    pub current_ac: i32,
    // #[serde(flatten)]
    // pub creature: CreatureType,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Data {
    pub players: Vec<Player>,
    pub parties: Vec<Party>,
    pub state: State,
}

#[derive(Event)]
pub enum TrackerEvent {
    TurnUpdate(Creature),
}

#[derive(Component, Default)]
pub struct Tracker {
    pub ordered: Vec<Creature>,
}

impl Tracker {
    fn get_turn_event(&self, new_state: &[Creature]) -> Option<TrackerEvent> {
        let current_turn = self.ordered.iter().find(|c| c.active);
        let new_turn = new_state.iter().find(|c| c.active);

        if current_turn != new_turn {
            if let Some(c) = new_turn {
                return Some(TrackerEvent::TurnUpdate(c.clone()));
            }
        }

        None
    }
}

#[derive(Component)]
pub struct TrackerOrdered;

pub fn send_request(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ReqTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        let req = reqwest::Request::new(
            reqwest::Method::GET,
            "http://127.0.0.1:8080/tracker/ordered".try_into().unwrap(),
        );

        commands.spawn((ReqwestRequest::new(req), TrackerOrdered));
    }
}

pub fn handle_response(
    mut commands: Commands,
    mut event_writer: EventWriter<TrackerEvent>,
    mut tracker_q: Query<&mut Tracker>,
    mut results: Query<(Entity, &ReqwestBytesResult), With<TrackerOrdered>>,
) {
    let mut tracker = tracker_q.single_mut();
    for (e, res) in results.iter_mut() {
        match &res.0 {
            Ok(_) => {
                let ordered_data = res
                    .deserialize_json::<Vec<Creature>>()
                    .expect("Failed to get new tracker order");
                log::debug!("{:?}", ordered_data);
                if tracker.ordered != ordered_data {
                    let e = tracker.get_turn_event(&ordered_data);
                    if let Some(e) = e {
                        event_writer.send(e);
                    }
                    tracker.ordered = ordered_data;
                }
            }
            Err(e) => log::error!("{:?}", e),
        }

        // Remove the old request
        commands.entity(e).despawn_recursive();
    }
}
