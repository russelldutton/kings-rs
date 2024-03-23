use itertools::Itertools;

use crate::models::{card_model::CardModel, rank::Rank, suit::Suit};

pub fn generate_deck() -> Vec<CardModel> {
    let suits: Vec<Suit> = vec![Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
    let ranks: Vec<Rank> = vec![
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];

    suits
        .iter()
        .flat_map(|s| ranks.iter().map(move |r| (s, r)))
        .map(|(suit, rank)| CardModel {
            rank: *rank,
            suit: *suit,
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::generate_deck;

    #[test]
    fn ensure_full_deck() {
        let deck = generate_deck();

        assert_eq!(deck.len(), 52);
    }
}
