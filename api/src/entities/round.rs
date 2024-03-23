use serde::Serialize;

use crate::models::rank::Rank;

#[derive(Serialize)]
pub struct Round {
    pub id: i64,
    pub rank: Rank,
    pub hand_size: i64,
    pub game_id: i64,
}
