use std::sync::Arc;

use crate::{ common::app_state::AppState, core::player::{fetch_users, join_lobby}, models::user::User };
use axum::{ extract::{ Path, State }, routing::get, Json, Router };

//lobby/join/:nick_name
pub fn player_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/lobby/join/:nick_name", get(join_lobby_handler))
        .route("/users", get(fetch_all_users))
}

async fn join_lobby_handler(
    Path(nick_name): Path<String>,
    State(state): State<Arc<AppState>>
) -> Result<Json<User>, String> {
    match join_lobby(&state.db_url, nick_name).await {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

async fn fetch_all_users(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<User>>, String> {
    match fetch_users(&state.db_url).await {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string())
    }
}