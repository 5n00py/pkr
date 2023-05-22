use std::fmt;

/// Errors that can occur when creating a `Suit` from a string.
#[derive(Debug, Clone)]
pub struct InvalidSuitError {
    details: String,
}

impl InvalidSuitError {
    fn new(details: &str) -> InvalidSuitError {
        InvalidSuitError {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for InvalidSuitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for InvalidSuitError {}

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
    /// Creates a new `Suit` from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the suit identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::card::Suit;
    ///
    /// let s = Suit::new_from_string("h").unwrap();
    /// assert_eq!(s, Suit::Heart);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `InvalidSuitError` if the string does not match any suit.
    pub fn new_from_string(s: &str) -> Result<Self, InvalidSuitError> {
        match s {
            "h" => Ok(Suit::Heart),
            "d" => Ok(Suit::Diamond),
            "c" => Ok(Suit::Club),
            "s" => Ok(Suit::Spade),
            _ => Err(InvalidSuitError::new("Invalid suit identifier")),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_suit_from_string() {
        assert_eq!(Suit::new_from_string("h").unwrap(), Suit::Heart);
        assert_eq!(Suit::new_from_string("d").unwrap(), Suit::Diamond);
        assert_eq!(Suit::new_from_string("c").unwrap(), Suit::Club);
        assert_eq!(Suit::new_from_string("s").unwrap(), Suit::Spade);
    }

    #[test]
    fn invalid_suit_from_string() {
        assert!(Suit::new_from_string("x").is_err());
    }
}
