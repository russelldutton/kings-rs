use itertools::Itertools;

use crate::{
    common::{app_error::AppError, app_state::Pool},
    models::{card_model::CardModel, player_model::PlayerModel, rank::Rank, suit::Suit},
};

pub async fn create_cards_in_game(pool: &Pool, players: Vec<PlayerModel>) -> Result<(), AppError> {
    let count: usize = players.iter().map(|p| p.hand.len()).sum();
    tracing::info!("Inserting {} cards into the game", count);

    struct InsertCardModel {
        pub rank: Rank,
        pub suit: Suit,
        pub game_id: i64,
        pub player_id: i64,
    }

    let insert_entries = players
        .iter()
        .flat_map(|player| {
            player
                .hand
                .iter()
                .map(|CardModel { rank, suit }| InsertCardModel {
                    game_id: player.game_id,
                    player_id: player.id,
                    rank: *rank,
                    suit: *suit,
                })
        })
        .collect_vec();

    let mut query_builder =
        sqlx::QueryBuilder::new("INSERT INTO cards (rank, suit, game_id, player_id) ");

    query_builder.push_values(insert_entries, |mut builder, card| {
        builder
            .push_bind(card.rank)
            .push_bind(card.suit)
            .push_bind(card.game_id)
            .push_bind(card.player_id);
    });

    let query = query_builder.build();

    query.execute(pool).await?;

    Ok(())
}
