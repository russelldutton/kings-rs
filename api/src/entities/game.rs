use super::{card::Card, player::Player, round::Round};

pub struct Game {
    pub id: i32,
    pub session_code: String,
    pub host: String, // player id
    pub is_started: bool,
    pub rounds: Vec<Round>,
    pub deck: Vec<Card>,
    pub players: Vec<Player>,
}
