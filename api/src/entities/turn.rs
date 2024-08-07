use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Turn {
    pub id: i64,
    pub player_id: i64,
    pub round_id: i64,
}
