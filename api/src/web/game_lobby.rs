use crate::{
    common::{app_error::AppError, app_state::AppState},
    data::{
        game::{create_game_lobby, get_game_by_session_code},
        player::create_player_in_game,
    },
    models::player_model::PlayerModel,
    util::{game_session_code::generate_random_code, user_session::get_user_id_from_session},
};
use axum::{
    extract::{Path, State},
    response::Result,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use tower_sessions::Session;
use tracing::instrument;

pub fn game_lobby_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", get(create_game_lobby_handler))
        .route("/join/:session_code", get(join_game_lobby_handler))
}

#[instrument]
async fn create_game_lobby_handler(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Json<PlayerModel>, AppError> {
    let user_id: i64 = get_user_id_from_session(session).await?;
    let session_code = generate_random_code();
    let game = create_game_lobby(&state.pool, user_id, session_code).await?;
    let player = create_player_in_game(&state.pool, user_id, game.id).await?;

    Ok(Json(PlayerModel {
        id: player.id,
        game_id: game.id,
        session_code: game.session_code,
        hand: vec![],
    }))
}

#[instrument]
pub async fn join_game_lobby_handler(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(session_code): Path<String>,
) -> Result<Json<PlayerModel>, AppError> {
    let user_id: i64 = get_user_id_from_session(session).await?;
    let game = get_game_by_session_code(&state.pool, session_code).await?;
    let player = create_player_in_game(&state.pool, user_id, game.id).await?;

    Ok(Json(PlayerModel {
        id: player.id,
        game_id: game.id,
        session_code: game.session_code,
        hand: vec![],
    }))
}
