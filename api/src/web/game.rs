use std::sync::Arc;
use tower_sessions::Session;

use crate::{
    common::app_state::AppState, core::game::{create_game_lobby, create_player}, models::player_model::PlayerModel, util::session_code::generate_random_code, USER_ID_KEY
};
use axum::{extract::State, routing::get, Json, Router};

pub fn game_routes() -> Router<Arc<AppState>> {
    Router::new().route("/game/lobby/create", get(create_game_lobby_handler))
    // .route("/game/lobby/join/:session_code", get(join_game_lobby_handler))
}

async fn create_game_lobby_handler(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Json<PlayerModel>, String> {
    let user_id: i64 = session.get(USER_ID_KEY).await.unwrap().unwrap();

    let session_code = generate_random_code();

    let game_result = create_game_lobby(&state.db_url, user_id, session_code).await;

    if let Err(err) = game_result {
        return Err(err.to_string());
    }

    let game = game_result.unwrap();

    let player_result = create_player(&state.db_url, user_id, game.id).await;

    if let Err(err) = player_result {
        return Err(err.to_string());
    }

    let player = player_result.unwrap();

    Ok(Json(PlayerModel {
        id: player.id,
        game_id: game.id,
        session_code: game.session_code
    }))
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
