use serde::Serialize;

use crate::models::role::Role;

#[derive(Serialize, Debug)]
pub struct PlayerInRound {
    pub id: i64,
    pub player_id: i64,
    pub round_id: i64,
    pub is_out: bool,
    pub next_role: Option<Role>,
}
