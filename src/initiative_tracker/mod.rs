use bevy::prelude::*;
use bevy_mod_reqwest::{reqwest, ReqwestBytesResult, ReqwestRequest};
use serde::Deserialize;

use crate::{token::TurnEvent, ReqTimer};

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
    currentAC: i32,
    currentHP: i32,
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

    #[serde(flatten)]
    pub creature: CreatureType,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct State {
    pub creatures: Vec<Creature>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Data {
    pub players: Vec<Player>,
    pub parties: Vec<Party>,
    pub state: State,
}

#[derive(Component)]
pub struct Tracker {
    pub data: Option<Data>,
}

pub fn send_request(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ReqTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        let req = reqwest::Request::new(
            reqwest::Method::GET,
            "http://127.0.0.1:8080/ttrpg_data".try_into().unwrap(),
        );

        commands.spawn(ReqwestRequest::new(req));
    }
}

pub fn handle_response(
    mut commands: Commands,
    mut event_writer: EventWriter<TurnEvent>,
    results: Query<(Entity, &ReqwestBytesResult)>,
    mut tracker_q: Query<&mut Tracker>,
) {
    let mut tracker = tracker_q.single_mut();
    for (e, res) in results.iter() {
        match &res.0 {
            Ok(_) => {
                let data = res
                    .deserialize_json::<Data>()
                    .expect("Failed to deserialize data");
            }
            Err(e) => log::error!("{:?}", e),
        }
        commands.entity(e).despawn_recursive();
    }
}
