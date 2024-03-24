use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use strum_macros::Display;

#[derive(Display, Debug)]
pub enum AppError {
    DbError(sqlx::Error),
    AxumError(axum::Error),
    ArgumentError(String),
    SessionError(tower_sessions::session::Error),
    InvalidGameState(String),
    Forbidden(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::DbError(db_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, db_error.to_string())
            }
            AppError::AxumError(axum_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, axum_error.to_string())
            }
            AppError::ArgumentError(message) => (StatusCode::BAD_REQUEST, message),
            AppError::SessionError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::InvalidGameState(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Forbidden(message) => (StatusCode::FORBIDDEN, message),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DbError(err)
    }
}

impl From<axum::Error> for AppError {
    fn from(err: axum::Error) -> Self {
        AppError::AxumError(err)
    }
}

impl From<tower_sessions::session::Error> for AppError {
    fn from(error: tower_sessions::session::Error) -> Self {
        AppError::SessionError(error)
    }
}
