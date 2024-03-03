use sqlx::SqlitePool;

pub async fn create_tables(db_url: &str) -> Result<(), sqlx::Error> {
    create_games_table(db_url).await?;
    create_rounds_table(db_url).await?;
    create_players_table(db_url).await?;
    create_turns_table(db_url).await?;
    create_cards_table(db_url).await?;
    create_swaps_table(db_url).await?;

    Ok(())
}

async fn create_cards_table(db_url: &str) -> Result<(), sqlx::Error> {
    let connection = SqlitePool::connect(db_url).await?;

    let query_str = "CREATE TABLE IF NOT EXISTS cards
    (
      id          TEXT  PRIMARY KEY  NOT NULL,
      rank        TEXT  NOT NULL,
      suit        TEXT  NOT NULL,
      game_id     TEXT,
      player_id   TEXT,
      turn_id     TEXT,
      FOREIGN KEY(game_id) REFERENCES games(game_id),
      FOREIGN KEY(player_id) REFERENCES players(player_id),
      FOREIGN KEY(turn_id) REFERENCES turns(turn_id)
    );";

    sqlx::query(&query_str).execute(&connection).await?;

    connection.close().await;

    Ok(())
}

async fn create_players_table(db_url: &str) -> Result<(), sqlx::Error> {
    let connection = SqlitePool::connect(db_url).await?;

    let query_str = "CREATE TABLE IF NOT EXISTS players
    (
      id          TEXT  PRIMARY KEY  NOT NULL,
      nickname    TEXT  NOT NULL,
      role        TEXT  NULL,
      game_id     TEXT,
      FOREIGN KEY(game_id) REFERENCES games(game_id)
    );";

    sqlx::query(&query_str).execute(&connection).await?;

    connection.close().await;

    Ok(())
}

async fn create_games_table(db_url: &str) -> Result<(), sqlx::Error> {
    let connection = SqlitePool::connect(db_url).await?;

    let query_str = "CREATE TABLE IF NOT EXISTS games
    (
      id            TEXT    PRIMARY KEY  NOT NULL,
      session_code  TEXT    UNIQUE       NOT NULL,
      player_host   TEXT    NOT NULL,
      is_started    BOOLEAN NOT NULL
    );";

    sqlx::query(&query_str).execute(&connection).await?;

    connection.close().await;

    Ok(())
}

async fn create_rounds_table(db_url: &str) -> Result<(), sqlx::Error> {
    let connection = SqlitePool::connect(db_url).await?;

    let query_str = "CREATE TABLE IF NOT EXISTS rounds
    (
      id            TEXT  PRIMARY KEY  NOT NULL,
      rank          TEXT  NOT NULL,
      hand_size     INTEGER  NOT NULL,
      game_id       TEXT NOT NULL,
      FOREIGN KEY(game_id) REFERENCES games(game_id)
    );";

    sqlx::query(&query_str).execute(&connection).await?;

    connection.close().await;

    Ok(())
}

async fn create_turns_table(db_url: &str) -> Result<(), sqlx::Error> {
    let connection = SqlitePool::connect(db_url).await?;

    let query_str = "CREATE TABLE IF NOT EXISTS turns
    (
      id            TEXT  PRIMARY KEY  NOT NULL,
      player_id     TEXT  NOT NULL,
      round_id      TEXT  NOT NULL,
      FOREIGN KEY(player_id) REFERENCES players(player_id),
      FOREIGN KEY(round_id) REFERENCES rounds(round_id)
    );";

    sqlx::query(&query_str).execute(&connection).await?;

    connection.close().await;

    Ok(())
}

async fn create_swaps_table(db_url: &str) -> Result<(), sqlx::Error> {
    let connection = SqlitePool::connect(db_url).await?;

    let query_str = "CREATE TABLE IF NOT EXISTS swaps
    (
      id            TEXT  PRIMARY KEY  NOT NULL,
      session_code  TEXT  NOT NULL,
      player_host   TEXT  NOT NULL
    );";

    sqlx::query(&query_str).execute(&connection).await?;

    connection.close().await;

    Ok(())
}
