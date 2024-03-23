use serde::Serialize;

#[derive(Serialize, sqlx::Type, PartialEq, Copy, Clone)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
