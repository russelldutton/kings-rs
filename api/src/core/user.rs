use axum::Json;
use sqlx::SqlitePool;

use crate::entities::user::User;

pub async fn register_user(db_url: &str, nick_name: String) -> Result<Json<User>, sqlx::Error> {
    tracing::info!("New player with nick_name {}", nick_name);

    let connection = SqlitePool::connect(db_url).await?;

    let new_user: User = sqlx::query_as!(
        User,
        "INSERT INTO users (nick_name) VALUES (?) RETURNING id, nick_name",
        nick_name
    )
    .fetch_one(&connection).await?;

    tracing::info!("Created new player: {}", new_user);
    Ok(Json(new_user))
}

pub async fn fetch_users(db_url: &str) -> Result<Json<Vec<User>>, sqlx::Error> {
    tracing::info!("Fetching all users");

    let connection = SqlitePool::connect(db_url).await?;

    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users")
    .fetch_all(&connection).await?;

    tracing::info!("Returned {} users", users.len());

    Ok(Json(users))
}