use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Serialize, Deserialize)]
pub struct OpponentModel {
    pub id: i64,
    pub nick_name: String,
    pub role: Role,
    pub remaining_cards: i64,
}
