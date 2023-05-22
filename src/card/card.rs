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
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
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
