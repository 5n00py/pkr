use std::error::Error;

use strum_macros::EnumIter;

/// Represents the suit of a playing card in a standard 52-card deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, EnumIter)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
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
    /// let s = Suit::new_from_str("h").unwrap();
    /// assert_eq!(s, Suit::Heart);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Box<dyn Error>` if the string does not match
    /// any suit.
    pub fn new_from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        match s {
            "h" => Ok(Suit::Heart),
            "d" => Ok(Suit::Diamond),
            "c" => Ok(Suit::Club),
            "s" => Ok(Suit::Spade),
            _ => Err("Invalid suit identifier".into()),
        }
    }

    /// Creates a new instance of `Suit` from a numerical value.
    ///
    /// # Arguments
    ///
    /// * `num` - A usize representing a numerical value for a Suit.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Result<Suit, Box<dyn Error>>`.
    /// If `num` does not match any Suit, an error is returned.
    pub fn new_from_num(num: usize) -> Result<Self, Box<dyn Error>> {
        match num {
            0 => Ok(Suit::Club),
            1 => Ok(Suit::Diamond),
            2 => Ok(Suit::Heart),
            3 => Ok(Suit::Spade),
            _ => Err(format!("Invalid numerical value for Suit: {}", num).into()),
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
        assert_eq!(Suit::new_from_str("h").unwrap(), Suit::Heart);
        assert_eq!(Suit::new_from_str("d").unwrap(), Suit::Diamond);
        assert_eq!(Suit::new_from_str("c").unwrap(), Suit::Club);
        assert_eq!(Suit::new_from_str("s").unwrap(), Suit::Spade);
    }

    #[test]
    fn invalid_suit_from_str() {
        assert!(Suit::new_from_str("x").is_err());
    }
}
