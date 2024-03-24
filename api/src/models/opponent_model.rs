use serde::Serialize;

use super::role::Role;

#[derive(Serialize)]
pub struct OpponentModel {
    pub id: i64,
    pub nick_name: String,
    pub role: Role,
    pub remaining_cards: i64,
}
