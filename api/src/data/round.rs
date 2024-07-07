use itertools::Itertools;

use crate::{
    common::{app_error::AppError, app_state::Pool},
    data::{card, player::get_players_in_game},
    entities::{round::Round, turn::Turn},
    models::{card_model::CardModel, rank::Rank, suit::Suit, turn_model::TurnModel},
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

pub async fn get_current_round(pool: &Pool, game_id: i64) -> Result<Option<Round>, AppError> {
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

    tracing::info!("Round result: {:?}", round);

    Ok(round)
}

pub async fn get_round_turns(pool: &Pool, round_id: i64) -> Result<Vec<TurnModel>, AppError> {
    tracing::info!("Fetching turns in round id: {:?}", round_id);

    struct CardTurn {
        id: i64,
        player_id: i64,
        rank: Rank,
        suit: Suit,
    }

    let card_turns = sqlx::query_as!(
        CardTurn,
        r#"
        SELECT t.id, t.player_id, c.rank as `rank: Rank`, c.suit as `suit: Suit`
        FROM turns as t
        INNER JOIN cards as c ON t.id = c.turn_id
        WHERE t.round_id = ?
        "#,
        round_id
    )
    .fetch_all(pool)
    .await?;

    let turns = card_turns
        .iter()
        .group_by(|g| (g.id, g.player_id))
        .into_iter()
        .map(|(key, value)| TurnModel {
            id: key.0,
            player_id: key.1,
            cards: value
                .map(|c| CardModel {
                    rank: c.rank,
                    suit: c.suit,
                })
                .collect_vec(),
        })
        .collect();

    Ok(turns)
}

pub async fn get_round_players(pool: &Pool, round_id: i64) -> Result<Vec<i64>, AppError> {
    tracing::info!("Getting player ids in round {}", round_id);

    struct Id {
        id: i64,
    }

    let players = sqlx::query_as!(
        Id,
        r#"
        SELECT id
        FROM players_in_round
        WHERE round_id = ? AND is_out = false"#,
        round_id,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|i| i.id)
    .collect_vec();

    Ok(players)
}
