use crate::{
    common::{app_error::AppError, app_state::AppState},
    data::{
        card::create_cards_in_game,
        game::{get_current_round, get_game_by_id, update_game_status},
        player::{fetch_opponents, fetch_player_cards, fetch_player_in_game, get_players_in_game},
    },
    entities::game::GameStatus,
    models::{game_model::GameModel, player_model::PlayerModel},
    util::{deck::generate_deck, user_session::get_user_id_from_session},
};
use axum::{
    extract::{Path, State},
    response::Result,
    routing::get,
    Json, Router,
};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::Arc;
use tower_sessions::Session;

use super::game_lobby::game_lobby_routes;

pub fn game_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/game/lobby", game_lobby_routes())
        .route("/game/:id", get(fetch_game_state))
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

pub async fn fetch_game_state(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameModel>, AppError> {
    let game = get_game_by_id(&state.pool, game_id).await?;
    let players = fetch_opponents(&state.pool, game_id).await?;
    let round = get_current_round(&state.pool, game_id).await?;

    Ok(Json(GameModel {
        id: game_id,
        session_code: game.session_code,
        is_started: game.status != GameStatus::Created,
        current_round: round,
        players,
    }))
}

// Play turn will always assume there are cards to play, either new round or existing
pub async fn play_turn(
    State(state): State<Arc<AppState>>,
    session: Session,
    Path(game_id): Path<i64>,
    cards: Json<Vec<i64>>,
) -> Result<(), AppError> {
    if cards.len() == 0 {
        return Err(AppError::ArgumentError(String::from(
            "Cannot play turn without hand",
        )));
    }

    let user_id = get_user_id_from_session(session).await?;

    let player = fetch_player_in_game(&state.pool, game_id, user_id).await?;
    let cards = fetch_player_cards(&state.pool, player.id).await?;
    let current_round = get_current_round(&state.pool, game_id).await?;

    // Ensure it is player's turn
    // Ensure player owns cards
    // Ensure cards are all same rank

    // Fetch round information
    // Ensure cards are at least round rank
    // Ensure cards are hand size for round

    todo!()
}

// This function will handle ending a player's turn
pub async fn pass_turn(
    State(state): State<Arc<AppState>>,
    session: Session,
    Path(game_id): Path<i64>,
) -> Result<(), AppError> {
    todo!()
}
