use serde::Serialize;

#[derive(Debug)]
pub struct Game {
    pub id: i64,
    pub session_code: String,
    pub host: i64,
    pub status: GameStatus
}


#[derive(Debug, Clone, sqlx::Type, PartialEq, Serialize)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum GameStatus {
    Setup,
    InProgress,
    Ended
}