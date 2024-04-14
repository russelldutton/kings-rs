use axum::{response::Html, routing::get, Router};
use chrono::prelude::*;
use sqlx::SqlitePool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::common::app_state::AppState;

mod common;
mod data;
mod entities;
mod models;
mod simulation_test;
mod util;
mod web;

const DB_URL: &str = "sqlite://kings.db";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .pretty()
        .init();

    let pool_result = SqlitePool::connect(DB_URL).await;
    if let Err(_) = pool_result {
        panic!("Error acquiring db pool");
    }
    let pool = pool_result.unwrap();

    let state = Arc::new(AppState { pool });
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn create_app(state: Arc<AppState>) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(CompressionLayer::new().br(true).gzip(true))
        .layer(session_layer);

    Router::new()
        .route("/", get(health_check))
        .merge(web::user::user_routes())
        .merge(web::game::game_routes())
        .with_state(state)
        .layer(middlewares)
}

async fn health_check() -> Html<String> {
    let current_date = Utc::now();
    let return_str = format!("<h1>Healthy! {current_date}</h1>");
    Html(return_str)
}
