use crate::{
    common::{app_error::AppError, app_state::Pool},
    entities::player::Player,
    models::role::Role,
};

pub async fn get_players_in_game(pool: &Pool, game_id: i64) -> Result<Vec<Player>, AppError> {
    tracing::info!("Fetching list of players in game {}", game_id);

    let players = sqlx::query_as!(
        Player,
        r#"
        SELECT id, user_id, role AS `role?: Role`, game_id AS `game_id?: i64` FROM players
        WHERE game_id = ?
        "#,
        game_id
    )
    .fetch_all(pool)
    .await?;

    tracing::info!("Found {} players in game", players.len());
    Ok(players)
}
