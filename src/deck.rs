use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;

/// Represents a deck of standard 52 playing cards.
///
/// A deck can be shuffled and cards can be dealt from it.
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a new deck of 52 standard playing cards.
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in &[Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
            for rank in &[
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
                Rank::Ace,
            ] {
                cards.push(Card::new(*rank, *suit));
            }
        }
        Self { cards }
    }

    /// Shuffles the deck.
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deals the top card from the deck.
    ///
    /// Returns `None` if the deck is empty.
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        let mut deck = Deck::new();
        let original_deck = deck.cards.clone();

        deck.shuffle();

        // The deck still contains 52 unique cards after shuffling
        assert_eq!(deck.cards.len(), 52);

        // The deck is not in the same order as before
        assert_ne!(deck.cards, original_deck);
    }

    #[test]
    fn test_deal() {
        let mut deck = Deck::new();

        // Check that dealing cards reduces the deck size
        let card = deck.deal();
        assert_eq!(deck.cards.len(), 51);
        assert!(card.is_some());

        // Check that dealing cards eventually empties the deck
        for _ in 0..51 {
            let card = deck.deal();
            assert!(card.is_some());
        }
        assert_eq!(deck.cards.len(), 0);

        // Check that dealing from an empty deck returns None
        let card = deck.deal();
        assert!(card.is_none());
    }
}
