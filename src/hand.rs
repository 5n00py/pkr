use crate::card::Card;

use std::fmt;

/// Errors that can occur when creating a `Hand`.
#[derive(Debug, Clone)]
pub struct InvalidHandError {
    details: String,
}

impl InvalidHandError {
    fn new(details: &str) -> InvalidHandError {
        InvalidHandError{details: details.to_string()}
    }
}

impl fmt::Display for InvalidHandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for InvalidHandError {}

/// Represents a poker hand.
///
/// A poker hand consists of 5 to 7 cards.
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Creates a new `Hand` from a vector of cards.
    ///
    /// # Arguments
    ///
    /// * `cards` - A vector of `Card`s.
    ///
    /// # Errors
    ///
    /// Returns an `InvalidHandError` if the number of cards is not 5, 6, or 7.
    pub fn new(cards: Vec<Card>) -> Result<Hand, InvalidHandError> {
        let num_cards = cards.len();
        if num_cards < 5 || num_cards > 7 {
            return Err(InvalidHandError::new("A poker hand must have between 5 and 7 cards."));
        }
        
        Ok(Hand { cards })
    }
}

#[test]
fn test_create_hand() {
    let cards = vec![
        Card::new_from_string("2h").unwrap(),
        Card::new_from_string("3d").unwrap(),
        Card::new_from_string("4s").unwrap(),
        Card::new_from_string("5c").unwrap(),
        Card::new_from_string("6h").unwrap(),
        Card::new_from_string("7d").unwrap(),
        Card::new_from_string("8s").unwrap(),
    ];

    let hand = Hand::new(cards);

    assert!(hand.is_ok());

    let hand = hand.unwrap();
    assert!(hand.cards.len() == 7)
}

#[test]
fn test_create_hand_with_wrong_number_of_cards() {
    let cards = vec![
        Card::new_from_string("2h").unwrap(),
        Card::new_from_string("3d").unwrap(),
    ];

    let result = Hand::new(cards);
    assert!(result.is_err());
}