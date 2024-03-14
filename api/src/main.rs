use std::sync::Arc;

use axum::{ response::Html, routing::get, Router };

use crate::common::app_state::AppState;

mod core;
mod models;
mod web;
mod common;

const DB_URL: &str = "sqlite://kings.db";

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState { db_url: DB_URL.to_string() });

    let app = Router::new()
        .route("/", get(handler))
        .merge(web::player::player_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}
