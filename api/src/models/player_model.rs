use serde::Serialize;

use super::card_model::CardModel;

#[derive(Serialize)]
pub struct PlayerModel {
    pub id: i64,
    pub game_id: i64,
    pub hand: Vec<CardModel>,
}
