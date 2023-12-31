mod state;

use bevy::prelude::*;
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
    #[serde(default)]
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

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update).add_event::<TrackerEvent>();
    }
}

#[derive(Component, Default)]
pub struct Tracker {
    pub error: Option<String>,
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

// #[derive(Component)]
// pub struct TrackerOrdered;

// pub fn send_request(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ReqTimer>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         let req = reqwest::Request::new(
//             reqwest::Method::GET,
//             "http://127.0.0.1:8080/tracker/ordered".try_into().unwrap(),
//         );

//         commands.spawn((ReqwestRequest::new(req), TrackerOrdered));
//     }
// }

// pub fn handle_response(
//     mut commands: Commands,
//     mut event_writer: EventWriter<TrackerEvent>,
//     mut tracker_q: Query<&mut Tracker>,
//     mut results: Query<(Entity, &ReqwestBytesResult), With<TrackerOrdered>>,
// ) {
//     let mut tracker = tracker_q.single_mut();
//     for (e, res) in results.iter_mut() {
//         match &res.0 {
//             Ok(_) => match res.deserialize_json::<Vec<Creature>>() {
//                 Some(ordered_data) => {
//                     if tracker.ordered != ordered_data {
//                         let e = tracker.get_turn_event(&ordered_data);
//                         if let Some(e) = e {
//                             event_writer.send(e);
//                         }
//                         tracker.ordered = ordered_data;
//                     }
//                     tracker.error = None;
//                 }
//                 None => {
//                     tracker.error = format!("Failed to deserialize data {:?}", res.as_str()).into()
//                 }
//             },
//             Err(e) => tracker.error = Some(e.to_string()),
//         }

//         // Remove the old request
//         commands.entity(e).despawn_recursive();
//     }
// }
