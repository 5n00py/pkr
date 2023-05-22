use std::fmt;

/// Errors that can occur when creating a `Rank` from a string.
#[derive(Debug, Clone)]
pub struct InvalidRankError {
    details: String,
}

impl InvalidRankError {
    fn new(details: &str) -> InvalidRankError {
        InvalidRankError {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for InvalidRankError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for InvalidRankError {}

/// Represents the rank of a playing card in a standard 52-card deck.
///
/// The ranks are represented as enum variants, from Two to Ace. The numerical
/// ordering of the variants represents the usual ordering of card ranks in most
/// games, with Two being the lowest and Ace being the highest.
///
/// # Examples
///
/// ```
/// use crate::pkr::card::Rank;
///
/// let rank = Rank::Ace;
/// assert_eq!(rank, Rank::Ace);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    /// Creates a new `Rank` from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the rank identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::card::Rank;
    ///
    /// let r = Rank::rank_from_string("A").unwrap();
    /// assert_eq!(r, Rank::Ace);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `InvalidRankError` if the string does not match any rank.
    pub fn rank_from_string(s: &str) -> Result<Self, InvalidRankError> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err(InvalidRankError::new("Invalid rank identifier")),
        }
    }

    /// Returns a string slice representing the `Rank`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::pkr::card::Rank;
    ///
    /// let rank = Rank::Two;
    /// assert_eq!(rank.as_str(), "2");
    ///
    /// let rank = Rank::Three;
    /// assert_eq!(rank.as_str(), "3");
    ///
    /// let rank = Rank::Ace;
    /// assert_eq!(rank.as_str(), "A");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_rank_from_string() {
        assert_eq!(Rank::rank_from_string("A").unwrap(), Rank::Ace);
        assert_eq!(Rank::rank_from_string("K").unwrap(), Rank::King);
        assert_eq!(Rank::rank_from_string("Q").unwrap(), Rank::Queen);
        assert_eq!(Rank::rank_from_string("J").unwrap(), Rank::Jack);
        assert_eq!(Rank::rank_from_string("T").unwrap(), Rank::Ten);
        assert_eq!(Rank::rank_from_string("2").unwrap(), Rank::Two);
    }

    #[test]
    fn invalid_rank_from_string() {
        assert!(Rank::rank_from_string("x").is_err());
    }
}
