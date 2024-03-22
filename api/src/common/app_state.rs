use sqlx::Sqlite;

pub type Pool = sqlx::Pool<Sqlite>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool,
}
