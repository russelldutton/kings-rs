use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type, PartialEq, Copy, Clone)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl From<String> for Suit {
    fn from(value: String) -> Self {
        match value.as_str() {
            "hearts" => Suit::Hearts,
            "diamonds" => Suit::Diamonds,
            "spades" => Suit::Spades,
            "clubs" => Suit::Clubs,
            _ => Suit::Hearts,
        }
    }
}
