use sqlx::SqlitePool;

use crate::entities::{game::{Game, GameStatus}, player::Player, role::Role};

pub async fn create_game_lobby(db_url: &str, user_id: i64, session_code: String) -> Result<Game, sqlx::Error> {
    tracing::info!("Creating new game with user id {} as host", user_id);

    let connection = SqlitePool::connect(db_url).await?;

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
        .fetch_one(&connection).await?;

    tracing::info!("Created new game: {:?}", new_game);
    Ok(new_game)
}

pub async fn create_player(db_url: &str, user_id: i64, game_id: i64) -> Result<Player, sqlx::Error> {
    tracing::info!("Looking for players with user_id: {}", user_id);

    let connection = SqlitePool::connect(db_url).await?;

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
    .fetch_one(&connection)
    .await?;

    Ok(player)
}
