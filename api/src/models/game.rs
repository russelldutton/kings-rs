use super::{card::Card, player::Player, round::Round};

pub struct Game {
    pub session_code: String,
    pub host: String,
    pub is_started: bool,
    pub rounds: Vec<Round>,
    pub deck: Vec<Card>,
    pub players: Vec<Player>,
}
