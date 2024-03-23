use serde::Serialize;

#[derive(Serialize, sqlx::Type, PartialEq, Copy, Clone)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Rank {
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Two,
    Joker,
}
