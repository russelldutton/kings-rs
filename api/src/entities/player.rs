use serde::Serialize;

use super::role::Role;

#[derive(Serialize)]
pub struct Player {
    pub id: i64,
    pub user_id: i64,
    pub role: Option<Role>,
    pub game_id: Option<i64>,
}
