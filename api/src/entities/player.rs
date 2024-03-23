use serde::Serialize;

use crate::models::role::Role;

#[derive(Serialize, Debug)]
pub struct Player {
    pub id: i64,
    pub user_id: i64,
    pub role: Option<Role>,
    pub game_id: Option<i64>,
}
