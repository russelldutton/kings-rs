use crate::core::player::join_lobby;
use axum::{routing::get, Router};

//lobby/join/:nick_name
pub fn player_routes() -> Router {
    Router::new().route("/lobby/join/:nick_name", get(join_lobby))
}
