use serde::Serialize;

use crate::models::rank::Rank;

#[derive(Serialize, Debug)]
pub struct Round {
    pub id: i64,
    pub rank: Rank,
    pub hand_size: i64,
    pub game_id: i64,
    pub is_ended: bool,
}
