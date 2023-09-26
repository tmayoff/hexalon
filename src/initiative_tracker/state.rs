use serde::Deserialize;

use super::Creature;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct State {
    pub creatures: Vec<Creature>,
}
