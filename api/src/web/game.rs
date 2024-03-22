use crate::{
    common::{app_error::AppError, app_state::AppState},
    core::game::{
        create_game_lobby, create_player_in_game, get_game_by_id, get_game_by_session_code,
    },
    entities::game::Game,
    models::player_model::PlayerModel,
    util::{game_session_code::generate_random_code, user_session::get_user_id_from_session},
    USER_ID_KEY,
};
use axum::{
    extract::{Path, Query, State},
    response::Result,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_sessions::Session;
use tracing::instrument;

pub fn game_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/game/lobby/create", get(create_game_lobby_handler))
        .route(
            "/game/lobby/join/:session_code",
            get(join_game_lobby_handler),
        )
        .route("/game", get(get_game_by))
}

#[instrument]
async fn create_game_lobby_handler(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Json<PlayerModel>, AppError> {
    let user_id: i64 = get_user_id_from_session(session).await?;

    let session_code = generate_random_code();

    let game = create_game_lobby(&state.db_url, user_id, session_code).await?;

    let player = create_player_in_game(&state.db_url, user_id, game.id).await?;

    Ok(Json(PlayerModel {
        id: player.id,
        game_id: game.id,
        session_code: game.session_code,
    }))
}

#[instrument]
pub async fn join_game_lobby_handler(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(session_code): Path<String>,
) -> Result<Json<PlayerModel>, AppError> {
    let user_id: i64 = get_user_id_from_session(session).await?;

    let game = get_game_by_session_code(&state.db_url, session_code).await?;

    let player = create_player_in_game(&state.db_url, user_id, game.id).await?;

    Ok(Json(PlayerModel {
        id: player.id,
        game_id: game.id,
        session_code: game.session_code,
    }))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSearch {
    pub session: Option<String>,
    pub id: Option<i64>,
}

#[instrument]
pub async fn get_game_by(
    State(state): State<Arc<AppState>>,
    Query(search): Query<GameSearch>,
) -> Result<Json<Game>, AppError> {
    if let Some(game_id) = search.id {
        let game = get_game_by_id(&state.db_url, game_id).await?;

        Ok(Json(game))
    } else if let Some(session_code) = search.session {
        let game = get_game_by_session_code(&state.db_url, session_code).await?;

        Ok(Json(game))
    } else {
        Err(AppError::ArgumentError(
            "No filter arguments supplied".to_string(),
        ))
    }
}
