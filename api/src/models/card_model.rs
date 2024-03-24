use serde::{Deserialize, Serialize};

use super::{rank::Rank, suit::Suit};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct CardModel {
    pub rank: Rank,
    pub suit: Suit,
}
