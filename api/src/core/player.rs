use axum::{extract, http::StatusCode, response::IntoResponse, Json};

pub async fn join_lobby(extract::Path(nick_name): extract::Path<String>) -> impl IntoResponse {
    tracing::debug!("New player with nick_name {}", nick_name);

    (StatusCode::OK, Json(nick_name))
}
