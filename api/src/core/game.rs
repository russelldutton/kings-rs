use axum::Json;
use sqlx::SqlitePool;

use crate::entities::{ player::Player, role::Role };

pub async fn create_game_lobby_and_player(
    db_url: &str,
    user_id: i64
) -> Result<Json<Player>, sqlx::Error> {
    // tracing::info!("New player with nick_name {}", nick_name);

    // let connection = SqlitePool::connect(db_url).await?;

    // let new_user: User = sqlx
    //     ::query_as(
    //         User,
    //         "INSERT INTO users (nick_name) VALUES (?) RETURNING id, nick_name",
    //         nick_name
    //     )
    //     .fetch_one(&connection).await?;

    // tracing::info!("Created new player: {}", new_user);
    // Ok(Json(new_user))

    Ok(
        Json(Player {
            id: user_id,
            game_id: Some(1),
            nick_name: "Russ".to_string(),
            role: Some(Role::Commoner),
        })
    )
}
