use serde::Serialize;
use strum_macros::{Display, EnumString};

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Game {
    pub id: i64,
    pub session_code: String,
    pub host: i64,
    pub status: GameStatus,
}

#[derive(Debug, Clone, sqlx::Type, PartialEq, Serialize, EnumString, Display)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum GameStatus {
    Created,
    Preparation,
    Started,
    Ended,
}

impl From<String> for GameStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "created" => GameStatus::Created,
            "preparation" => GameStatus::Preparation,
            "started" => GameStatus::Started,
            "ended" => GameStatus::Ended,
            _ => GameStatus::Created,
        }
    }
}
