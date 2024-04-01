use serde::{Deserialize, Serialize};

use crate::models::{rank::Rank, suit::Suit};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: i64,
    pub rank: Rank,
    pub suit: Suit,
    pub game_id: i64,
    pub player_id: i64,
    pub turn_id: Option<i64>,
}
