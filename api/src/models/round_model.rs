use serde::{Deserialize, Serialize};

use crate::entities::round::Round;

use super::{rank::Rank, turn_model::TurnModel};

#[derive(Serialize, Deserialize)]
pub struct RoundModel {
    pub rank: Rank,
    pub hand_size: i64,
    pub turns: Vec<TurnModel>,
    pub players_remaining: Vec<i64>,
}

impl From<Round> for RoundModel {
    fn from(value: Round) -> Self {
        Self {
            hand_size: value.hand_size,
            rank: value.rank,
            players_remaining: vec![],
            turns: vec![],
        }
    }
}

impl From<Round> for Option<RoundModel> {
    fn from(value: Round) -> Self {
        Some(value.into())
    }
}

impl RoundModel {
    pub fn with_turns(mut self, turns: Vec<TurnModel>) -> Self {
        self.turns = turns;
        self
    }

    pub fn with_players(mut self, players: Vec<i64>) -> Self {
        self.players_remaining = players;
        self
    }
}
