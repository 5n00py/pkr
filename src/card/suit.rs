/// Represents the suit of a playing card in a standard 52-card deck.
///
/// The suits are represented as enum variants: Heart, Diamond, Club, and Spade.
///
/// # Examples
///
/// ```
/// use crate::pkr::card::Suit;
///
/// let suit = Suit::Heart;
/// assert_eq!(suit, Suit::Heart);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Suit {
    /// Returns a string slice representing the `Suit`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::pkr::card::Suit;
    ///
    /// let suit = Suit::Heart;
    /// assert_eq!(suit.as_str(), "h");
    ///
    /// let suit = Suit::Diamond;
    /// assert_eq!(suit.as_str(), "d");
    ///
    /// let suit = Suit::Club;
    /// assert_eq!(suit.as_str(), "c");
    ///
    /// let suit = Suit::Spade;
    /// assert_eq!(suit.as_str(), "s");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Suit::Heart => "h",
            Suit::Diamond => "d",
            Suit::Club => "c",
            Suit::Spade => "s",
        }
    }
}
