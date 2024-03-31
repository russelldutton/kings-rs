use serde::{Deserialize, Serialize};

use super::{opponent_model::OpponentModel, round_model::RoundModel};

#[derive(Serialize, Deserialize)]
pub struct GameModel {
    pub id: i64,
    pub session_code: String,
    pub is_started: bool,
    pub current_round: Option<RoundModel>,
    pub players: Vec<OpponentModel>,
}
