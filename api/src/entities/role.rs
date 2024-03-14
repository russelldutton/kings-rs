use serde::Serialize;

#[derive(Debug, Clone, sqlx::Type, PartialEq, Serialize)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Role {
    King,
    Queen,
    Jester,
    Peasant,
    Commoner,
}
