use std::sync::Arc;
use tower_sessions::Session;

use crate::{
    common::app_state::AppState,
    core::game::create_game_lobby_and_player,
    entities::player::Player,
    USER_ID_KEY,
};
use axum::{ extract::State, http::HeaderMap, routing::get, Json, Router };

pub fn game_routes() -> Router<Arc<AppState>> {
    Router::new().route("/game/lobby/create", get(create_game_lobby_handler))
    // .route("/game/lobby/join/:lobby_id", get(join_game_lobby_handler))
}

async fn create_game_lobby_handler(
    session: Session,
    State(state): State<Arc<AppState>>
) -> Result<Json<Player>, String> {
    let user_id: i64 = session.get(USER_ID_KEY).await.unwrap().unwrap();

    match create_game_lobby_and_player(&state.db_url, user_id).await {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

// async fn join_game_lobby_handler(
//     headers: HeaderMap,
//     State(state): State<Arc<AppState>>
// ) -> Result<Json<Player>, String> {
//     match register_user(&state.db_url, user_id).await {
//         Ok(response) => Ok(response),
//         Err(err) => Err(err.to_string()),
//     }
// }
