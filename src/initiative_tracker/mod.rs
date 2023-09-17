use bevy::prelude::Component;
use serde::Deserialize;

// TODO consider cleaning these structs up. Especially when it comes to the initiative vs non initiative characters

#[derive(Debug, Deserialize)]
pub struct Player {
    name: String,
    ac: i32,
    hp: i32,
    level: i32,
    modifier: i32,
}

#[derive(Debug, Deserialize)]
pub struct Monster {
    name: String,
    display: Option<String>,
    ac: i32,
    hp: i32,
    cr: String,
    currentAC: i32,
    currentHP: i32,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Party {
    pub name: String,
    pub players: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CreatureType {
    Player(Player),
    Monster(Monster),
}

#[derive(Debug, Deserialize)]
pub struct Creature {
    pub id: String,
    pub initiative: i32,
    pub player: Option<bool>,

    #[serde(flatten)]
    pub creature: CreatureType,
}

#[derive(Debug, Deserialize)]
pub struct State {
    pub creatures: Vec<Creature>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub players: Vec<Player>,
    pub parties: Vec<Party>,
    pub state: State,
}

#[derive(Component)]
pub struct Tracker {
    pub data: Option<Data>,
}
