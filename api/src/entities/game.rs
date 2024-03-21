use strum_macros::EnumString;
use serde::Serialize;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Game {
    pub id: i64,
    pub session_code: String,
    pub host: i64,
    pub status: GameStatus
}


#[derive(Debug, Clone, sqlx::Type, PartialEq, Serialize, EnumString)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum GameStatus {
    Setup,
    Started,
    Ended
}

impl From<String> for GameStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ended" => GameStatus::Ended,
            "started" => GameStatus::Started,
            "setup" => GameStatus::Setup,
            _ => GameStatus::Setup
        }
    }
}