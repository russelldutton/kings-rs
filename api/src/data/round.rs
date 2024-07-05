use itertools::Itertools;

use crate::{
    common::{app_error::AppError, app_state::Pool},
    data::player::get_players_in_game,
    entities::round::Round,
    models::rank::Rank,
};

pub async fn create_new_round(
    pool: &Pool,
    game_id: i64,
    rank: Rank,
    hand_size: i64,
) -> Result<Round, AppError> {
    tracing::info!(
        "Creating new round with rank {} and hand size {}",
        rank,
        hand_size
    );

    let new_round = sqlx::query_as!(
        Round,
        r#"
          INSERT INTO rounds (rank, hand_size, game_id, is_ended)
          VALUES (?, ?, ?, ?)
          RETURNING id, rank, hand_size, game_id, is_ended
          "#,
        rank,
        hand_size,
        game_id,
        false
    )
    .fetch_one(pool)
    .await?;

    let players = get_players_in_game(pool, game_id)
        .await?
        .iter()
        .map(|player| player.id)
        .collect_vec();

    let mut query_builder =
        sqlx::QueryBuilder::new("INSERT INTO players_in_round (player_id, round_id, is_out) ");

    query_builder.push_tuples(players, |mut builder, id| {
        builder
            .push_bind(id)
            .push_bind(new_round.id)
            .push_bind(false);
    });

    let query = query_builder.build();

    query.execute(pool).await?;

    tracing::info!("Created new game: {:?}", new_round);
    Ok(new_round)
}

pub async fn get_active_round(pool: &Pool, game_id: i64) -> Result<Option<Round>, AppError> {
    tracing::info!("Searching for active round in game {}", game_id);

    let round = sqlx::query_as!(
        Round,
        r#"
        SELECT id, rank, hand_size, game_id, is_ended
        FROM rounds
        WHERE game_id = ? AND is_ended = false
        "#,
        game_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(round)
}
