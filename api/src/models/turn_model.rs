use serde::{Deserialize, Serialize};

use super::card_model::CardModel;

#[derive(Serialize, Deserialize)]
pub struct TurnModel {
    pub id: i64,
    pub player_id: i64,
    pub cards: Vec<CardModel>,
}
