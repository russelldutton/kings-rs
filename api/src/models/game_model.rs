use serde::Serialize;

use crate::entities::{card::Card, player::Player, round::Round};

#[derive(Serialize)]
pub struct GameModel {
    pub id: i32,
    pub session_code: String,
    pub is_started: bool,
    pub rounds: Vec<Round>,
    pub deck: Vec<Card>,
    pub players: Vec<Player>,
}
