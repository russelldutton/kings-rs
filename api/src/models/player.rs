use super::{game::Game, role::Role};

pub struct Player {
    pub nickname: String,
    pub role: Role,
    pub game: Game,
}
