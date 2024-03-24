use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type, PartialEq, Copy, Clone)]
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

impl From<String> for Rank {
    fn from(value: String) -> Self {
        match value.as_str() {
            "three" => Rank::Three,
            "four" => Rank::Four,
            "five" => Rank::Five,
            "six" => Rank::Six,
            "seven" => Rank::Seven,
            "eight" => Rank::Eight,
            "nine" => Rank::Nine,
            "ten" => Rank::Ten,
            "jack" => Rank::Jack,
            "queen" => Rank::Queen,
            "king" => Rank::King,
            "ace" => Rank::Ace,
            "two" => Rank::Two,
            "joker" => Rank::Joker,
            _ => Rank::Three,
        }
    }
}
