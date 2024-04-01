use crate::{
    common::{app_error::AppError, app_state::AppState},
    data::user::{fetch_users, register_user},
    entities::user::User,
    util::user_session::add_user_id_to_session,
};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use tower_sessions::Session;

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/user/register/:nick_name", get(register_user_handler))
        .route("/users", get(get_users_handler))
}

async fn register_user_handler(
    Path(nick_name): Path<String>,
    State(state): State<Arc<AppState>>,
    session: Session,
) -> Result<Json<User>, AppError> {
    let user = register_user(&state.pool, nick_name).await?;
    add_user_id_to_session(session, user.id).await?;
    Ok(Json(user))
}

async fn get_users_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, AppError> {
    Ok(Json(fetch_users(&state.pool).await?))
}
