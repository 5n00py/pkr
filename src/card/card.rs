use std::error;
use std::fmt;

use super::Rank;
use super::Suit;

/// Represents a playing card with a rank and suit in a standard 52-card deck.
///
/// A card is a combination of a rank and a suit.
///
/// # Examples
///
/// ```
/// use crate::pkr::card::Rank;
/// use crate::pkr::card::Suit;
/// use crate::pkr::card::Card;
///
/// let card = Card::new(Rank::Ace, Suit::Spade);
/// assert_eq!(card, Card::new(Rank::Ace, Suit::Spade));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    /// Creates a new `Card` from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the card identifier.
    ///         The first character represents the rank and the second
    ///         represents the suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::card::{Card, Rank, Suit};
    ///
    /// let card = Card::new_from_str("Ac").unwrap();
    /// assert_eq!(card, Card { rank: Rank::Ace, suit: Suit::Club });
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn std::error::Error>` if the string does not match
    /// any card, the rank or the suit are invalid.
    pub fn new_from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if s.len() != 2 {
            return Err("Card string must be of length 2".into());
        }

        let rank = Rank::new_from_str(&s[0..1])?;
        let suit = Suit::new_from_str(&s[1..2])?;

        Ok(Self { rank, suit })
    }

    /// Returns a string representation of the `Card`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::pkr::card::Rank;
    /// use crate::pkr::card::Suit;
    /// use crate::pkr::card::Card;
    ///
    /// let card = Card::new(Rank::Ace, Suit::Spade);
    /// assert_eq!(card.as_str(), "As");
    ///
    /// let card = Card::new(Rank::Two, Suit::Club);
    /// assert_eq!(card.as_str(), "2c");
    /// ```
    pub fn as_str(&self) -> String {
        format!("{}{}", self.rank.as_str(), self.suit.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_card_from_string() {
        let card = Card::new_from_str("Ac").unwrap();
        assert_eq!(
            card,
            Card {
                rank: Rank::Ace,
                suit: Suit::Club
            }
        );

        let card = Card::new_from_str("Td").unwrap();
        assert_eq!(
            card,
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamond
            }
        );

        let card = Card::new_from_str("3s").unwrap();
        assert_eq!(
            card,
            Card {
                rank: Rank::Three,
                suit: Suit::Spade
            }
        );
    }

    #[test]
    fn new_card_from_invalid_string() {
        assert!(Card::new_from_str("AcA").is_err());
        assert!(Card::new_from_str("M").is_err());
        assert!(Card::new_from_str("As1").is_err());
        assert!(Card::new_from_str("1c").is_err());
        assert!(Card::new_from_str("").is_err());
    }
}
