use serde::Deserialize;

use super::Creature;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct State {
    pub creatures: Vec<Creature>,
}

impl State {
    pub fn current_creatures_turn(&self) -> Option<Creature> {
        self.creatures.iter().find(|c| c.active).cloned()
    }
}
