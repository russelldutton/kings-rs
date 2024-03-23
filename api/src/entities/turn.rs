use serde::Serialize;

#[derive(Serialize)]
pub struct Turn {
    pub id: i64,
    pub player_id: i64,
    pub round_id: i64,
}
