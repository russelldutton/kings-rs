use super::{card::Card, game::Game, role::Role};

pub struct Player {
    pub id: String,
    pub nickname: String,
    pub role: Role,
    pub hand: Vec<Card>,
}
