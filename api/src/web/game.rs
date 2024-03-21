use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_sessions::Session;
use tracing::instrument;

use crate::{
    common::app_state::AppState,
    core::game::{
        create_game_lobby, create_player_in_game, get_game_by_id, get_game_by_session_code,
    },
    entities::game::Game,
    models::player_model::PlayerModel,
    util::session_code::generate_random_code,
    USER_ID_KEY,
};
use axum::{
    extract::{Path, Query, State},
    response::Result,
    routing::get,
    Json, Router,
};

pub fn game_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/game/lobby/create", get(create_game_lobby_handler))
        .route(
            "/game/lobby/join/:session_code",
            get(join_game_lobby_handler),
        )
        .route("/game", get(get_game_by))
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

    let player_result = create_player_in_game(&state.db_url, user_id, game.id).await;

    if let Err(err) = player_result {
        return Err(err.to_string());
    }

    let player = player_result.unwrap();

    Ok(Json(PlayerModel {
        id: player.id,
        game_id: game.id,
        session_code: game.session_code,
    }))
}

pub async fn join_game_lobby_handler(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(session_code): Path<String>,
) -> Result<Json<PlayerModel>, String> {
    let user_id: i64 = session.get(USER_ID_KEY).await.unwrap().unwrap();

    let game_result = get_game_by_session_code(&state.db_url, session_code).await;
    if let Err(err) = game_result {
        return Err(err.to_string());
    }

    let game = game_result.unwrap();

    let player_result = create_player_in_game(&state.db_url, user_id, game.id).await;
    if let Err(err) = player_result {
        return Err(err.to_string());
    }

    let player = player_result.unwrap();

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
) -> Result<Json<Game>, String> {
    if let Some(game_id) = search.id {
        let game = get_game_by_id(&state.db_url, game_id).await.unwrap();

        Ok(Json(game))
    } else if let Some(session_code) = search.session {
        let game = get_game_by_session_code(&state.db_url, session_code)
            .await
            .unwrap();

        Ok(Json(game))
    } else {
        Err("No search parameters provided".to_string())
    }
}
