use bevy::prelude::Component;
use serde::Deserialize;

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
