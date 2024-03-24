use crate::{
    common::{app_error::AppError, app_state::AppState},
    core::{
        card::create_cards_in_game,
        game::{get_game_by_id, get_game_by_session_code, update_game_status},
        player::get_players_in_game,
    },
    entities::game::{Game, GameStatus},
    models::player_model::PlayerModel,
    util::{deck::generate_deck, user_session::get_user_id_from_session},
};
use axum::{
    extract::{Path, Query, State},
    response::Result,
    routing::get,
    Json, Router,
};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_sessions::Session;
use tracing::instrument;

use super::game_lobby::game_lobby_routes;

pub fn game_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/game/lobby", game_lobby_routes())
        .route("/game", get(get_game_by))
        .route("/game/:id/prepare", get(prepare_game))
}

pub async fn prepare_game(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<(), AppError> {
    let user_id = get_user_id_from_session(session).await?;

    let game = get_game_by_id(&state.pool, game_id).await?;
    if game.status != GameStatus::Created {
        return Err(AppError::InvalidGameState(
            "Cannot prepare game not in Created status".to_string(),
        ));
    }

    if game.host != user_id {
        return Err(AppError::Forbidden(
            "Only the game host may start the game.".to_string(),
        ));
    }

    update_game_status(&state.pool, game_id, GameStatus::Preparation).await?;
    let players = get_players_in_game(&state.pool, game_id).await?;

    let mut new_deck = generate_deck();
    new_deck.shuffle(&mut thread_rng());

    let starting_hand_size = new_deck.len() / players.len();

    let player_models = new_deck
        .iter()
        .chunks(starting_hand_size)
        .into_iter()
        .zip(players)
        .map(|(cards, player)| PlayerModel {
            game_id,
            id: player.id,
            session_code: game.session_code.clone(),
            hand: cards.cloned().collect_vec(),
        })
        .collect_vec();

    create_cards_in_game(&state.pool, player_models).await?;

    Ok(())
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
        Ok(Json(get_game_by_id(&state.pool, game_id).await?))
    } else if let Some(session_code) = search.session {
        Ok(Json(
            get_game_by_session_code(&state.pool, session_code).await?,
        ))
    } else {
        Err(AppError::ArgumentError(
            "No filter arguments supplied".to_string(),
        ))
    }
}
