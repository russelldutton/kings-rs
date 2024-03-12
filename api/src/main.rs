use axum::{response::Html, routing::get, Router};
use data::migrations::create_tables;
use sqlx::{migrate::MigrateDatabase, Sqlite};

mod core;
mod data;
mod models;
mod web;

const DB_URL: &str = "sqlite://kings.db";

#[tokio::main]
async fn main() {
    if Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("DB already exists. Dropping DB to run migrations");
        Sqlite::drop_database(DB_URL);
        println!("Creating KINGS DB {}", DB_URL);
    }

    match Sqlite::create_database(DB_URL).await {
        Ok(_) => println!("Creation successful!"),
        Err(error) => panic!("Error: {}", error),
    }

    create_tables(DB_URL).await.unwrap();

    println!("Migrations completed successfully");

    let app = Router::new()
        .route("/", get(handler))
        .merge(web::player::player_routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}
