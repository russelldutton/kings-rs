use serde::{Deserialize, Serialize};

use super::card_model::CardModel;

#[derive(Serialize, Deserialize)]
pub struct PlayerModel {
    pub id: i64,
    pub game_id: i64,
    pub session_code: String,
    pub hand: Vec<CardModel>,
}
