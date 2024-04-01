use crate::{common::app_state::Pool, entities::user::User};

pub async fn register_user(pool: &Pool, nick_name: String) -> Result<User, sqlx::Error> {
    tracing::info!("New player with nick_name {}", nick_name);

    let new_user: User = sqlx::query_as!(
        User,
        "INSERT INTO users (nick_name) VALUES (?) RETURNING id, nick_name",
        nick_name
    )
    .fetch_one(pool)
    .await?;

    tracing::info!("Created new player: {}", new_user);
    Ok(new_user)
}

pub async fn fetch_users(pool: &Pool) -> Result<Vec<User>, sqlx::Error> {
    tracing::info!("Fetching all users");

    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool)
        .await?;

    tracing::info!("Returned {} users", users.len());

    Ok(users)
}
