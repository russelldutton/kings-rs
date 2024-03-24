use crate::{
    common::{app_error::AppError, app_state::Pool},
    entities::player::Player,
    models::{opponent_model::OpponentModel, role::Role},
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

pub async fn fetch_opponents(pool: &Pool, game_id: i64) -> Result<Vec<OpponentModel>, AppError> {
    tracing::info!("Fetching list of players in game {}", game_id);

    let opponents = sqlx::query_as!(
        OpponentModel,
        "SELECT 
            u.nick_name AS nick_name,
            p.id AS id,
            p.role AS `role!: Role`,
            COUNT(c.id) AS remaining_cards
        FROM 
            users u
        JOIN 
            players p ON u.id = p.user_id
        JOIN 
            cards c ON p.id = c.player_id
        GROUP BY 
            u.nick_name, p.id, p.role;"
    )
    .fetch_all(pool)
    .await?;

    tracing::info!("Found {} players in game", opponents.len());
    Ok(opponents)
}
