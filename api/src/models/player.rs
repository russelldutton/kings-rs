use serde::Serialize;

use super::role::Role;

#[derive(Serialize)]
pub struct Player {
    pub id: i64,
    pub nick_name: String,
    pub role: Option<Role>,
    pub game_id: Option<i64>
}
