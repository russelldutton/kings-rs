use std::sync::Arc;
use tower::ServiceBuilder;
use axum::{ response::Html, routing::get, Router };
use tower_http::{ compression::CompressionLayer, cors::{ Any, CorsLayer }, trace::TraceLayer };
use tower_sessions::{ Expiry, MemoryStore, SessionManagerLayer };
use chrono::prelude::*;

use crate::common::app_state::AppState;

mod core;
mod entities;
mod web;
mod common;

const DB_URL: &str = "sqlite://kings.db";
pub const USER_ID_KEY: &str = "user_id";

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState { db_url: DB_URL.to_string() });

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnSessionEnd);

    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(CompressionLayer::new())
        .layer(session_layer);

    let app = Router::new()
        .route("/", get(handler))
        .merge(web::user::user_routes())
        .merge(web::game::game_routes())
        .with_state(state)
        .layer(middlewares);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    let current_date = Utc::now();
    let return_str = format!("<h1>Healthy! {current_date}</h1>");
    Html(return_str)
}
