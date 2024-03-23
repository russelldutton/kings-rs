use serde::Serialize;

#[derive(Serialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
