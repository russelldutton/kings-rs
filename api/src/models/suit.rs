use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
