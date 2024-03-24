use serde::Serialize;

use crate::entities::round::Round;

use super::rank::Rank;

#[derive(Serialize)]
pub struct RoundModel {
    pub rank: Rank,
    pub hand_size: i64,
}

impl From<Round> for RoundModel {
    fn from(value: Round) -> Self {
        Self {
            hand_size: value.hand_size,
            rank: value.rank,
        }
    }
}
