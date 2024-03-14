use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub nick_name: String
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: id {}, nick_name {}", self.id, self.nick_name)
    }
}