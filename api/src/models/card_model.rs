use serde::Serialize;

use super::{rank::Rank, suit::Suit};

#[derive(Serialize)]
pub struct CardModel {
    pub rank: Rank,
    pub suit: Suit,
}
