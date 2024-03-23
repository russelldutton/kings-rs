use serde::Serialize;

#[derive(Serialize)]
pub struct PlayerModel {
    pub id: i64,
    pub game_id: i64,
    pub session_code: String,
}
