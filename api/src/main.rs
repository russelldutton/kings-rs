use axum::{response::Html, routing::get, Router};
use chrono::prelude::*;
use tracing::instrument;
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
mod core;
mod entities;
mod web;
mod util;
mod models;

const DB_URL: &str = "sqlite://kings.db";
pub const USER_ID_KEY: &str = "user_id";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .pretty()
        .init();

    let state = Arc::new(AppState {
        db_url: DB_URL.to_string(),
    });

    let session_store = MemoryStore::default();
    // let on_inactivity = Expiry::OnInactivity(Duration::weeks(2));
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let middlewares = ServiceBuilder::new()
        // .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(CompressionLayer::new())
        .layer(session_layer);

    let app = Router::new()
        .route("/", get(handler))
        .merge(web::user::user_routes())
        .merge(web::game::game_routes())
        .with_state(state)
        .layer(middlewares);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    let current_date = Utc::now();
    let return_str = format!("<h1>Healthy! {current_date}</h1>");
    Html(return_str)
}
