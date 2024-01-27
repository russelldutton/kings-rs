use migrations::tables::create_tables;
use sqlx::{migrate::MigrateDatabase, Sqlite};

mod migrations;
mod models;

const DB_URL: &str = "sqlite://kings.db";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating KINGS DB {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Creation successful!"),
            Err(error) => panic!("Error: {}", error),
        }
    } else {
        println!("DB already exists.");
    }

    create_tables(DB_URL).await?;

    println!("Migrations completed successfully");

    Ok(())
}
