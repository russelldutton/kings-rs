use crate::{
    common::{app_error::AppError, app_state::Pool},
    entities::{card::Card, player::Player},
    models::{opponent_model::OpponentModel, role::Role},
};

pub async fn get_players_in_game(pool: &Pool, game_id: i64) -> Result<Vec<Player>, AppError> {
    tracing::info!("Fetching list of players in game {}", game_id);

    let players = sqlx::query_as!(
        Player,
        r#"
        SELECT id, user_id, role AS `role?: Role`, game_id AS `game_id?: i64` FROM players
        WHERE game_id = ?
        ORDER BY id ASC
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

pub async fn fetch_player_in_game(
    pool: &Pool,
    game_id: i64,
    user_id: i64,
) -> Result<Player, AppError> {
    tracing::info!(
        "Looking for player in game id {} with user id {}",
        game_id,
        user_id
    );

    let player = sqlx::query_as!(
        Player,
        "SELECT id, user_id, role as `role: Role`, game_id
        FROM players
        WHERE user_id = ? AND game_id = ?",
        user_id,
        game_id
    )
    .fetch_one(pool)
    .await?;

    Ok(player)
}

pub async fn fetch_player_cards(pool: &Pool, player_id: i64) -> Result<Vec<Card>, AppError> {
    tracing::info!("Looking for cards for player {}", player_id);

    let cards = sqlx::query_as!(
        Card,
        "Select * FROM cards
        WHERE player_id = ?",
        player_id
    )
    .fetch_all(pool)
    .await?;

    tracing::info!("Player {} has {} cards remaining", player_id, cards.len());
    Ok(cards)
}
