use crate::{
    common::app_state::Pool,
    entities::{
        game::{Game, GameStatus},
        player::Player,
        role::Role,
    },
};
use tracing::instrument;

pub async fn create_game_lobby(
    pool: &Pool,
    user_id: i64,
    session_code: String,
) -> Result<Game, sqlx::Error> {
    tracing::info!("Creating new game with user id {} as host", user_id);

    let new_game = sqlx::query_as!(
        Game,
        r#"
            INSERT INTO games (session_code, host, status)
            VALUES (?, ?, ?)
            RETURNING id, session_code, host, status AS `status: GameStatus`
            "#,
        session_code,
        user_id,
        GameStatus::Setup
    )
    .fetch_one(pool)
    .await?;

    tracing::info!("Created new game: {:?}", new_game);
    Ok(new_game)
}

pub async fn create_player_in_game(
    pool: &Pool,
    user_id: i64,
    game_id: i64,
) -> Result<Player, sqlx::Error> {
    tracing::info!("Looking for players with user_id: {}", user_id);

    let role = Role::Commoner;

    let player = sqlx::query_as!(
        Player,
        r#"
        INSERT INTO players (user_id, game_id, role) VALUES (?, ?, ?) RETURNING id, user_id, role AS `role?: Role`, game_id
        "#,
        user_id,
        game_id,
        role
    )
    .fetch_one(pool)
    .await?;

    tracing::info!("Created new player: {:?} in game id: {}", player, game_id);
    Ok(player)
}

pub async fn get_game_by_id(pool: &Pool, game_id: i64) -> Result<Game, sqlx::Error> {
    tracing::info!("Looking for game with id: {}", game_id);

    let game = sqlx::query_as!(
        Game,
        r#"
        SELECT id, session_code, host, status
        FROM games
        WHERE id = ?
        "#,
        game_id
    )
    .fetch_one(pool)
    .await?;

    tracing::info!("Found game by id: {:?}", game);
    Ok(game)
}

#[instrument]
pub async fn get_game_by_session_code(
    pool: &Pool,
    session_code: String,
) -> Result<Game, sqlx::Error> {
    tracing::info!("Looking for game with session code: {}", session_code);

    let game: Game = sqlx::query_as(
        r#"
        SELECT id, session_code, host, status
        FROM games
        WHERE session_code = ? AND status = ?
        "#,
    )
    .bind(session_code)
    .bind(GameStatus::Setup)
    .fetch_one(pool)
    .await?;

    tracing::info!("Found game by id: {:?}", game);
    Ok(game)
}
