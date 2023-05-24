use std::error::Error;

/// Represents the rank of a playing card in a standard 52-card deck.
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
    /// let r = Rank::new_from_str("A").unwrap();
    /// assert_eq!(r, Rank::Ace);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the string does not match any rank.
    pub fn new_from_str(s: &str) -> Result<Self, Box<dyn Error>> {
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
            _ => Err("Invalid rank identifier".into()),
        }
    }

    /// Creates a new instance of `Rank` from a numerical value.
    ///
    /// # Arguments
    ///
    /// * `num` - A usize representing a numerical value for a Rank.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Result<Rank, Box<dyn Error>>`.
    /// If `num` does not match any Rank, an error is returned.
    pub fn new_from_num(num: usize) -> Result<Self, Box<dyn Error>> {
        match num {
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            9 => Ok(Rank::Nine),
            10 => Ok(Rank::Ten),
            11 => Ok(Rank::Jack),
            12 => Ok(Rank::Queen),
            13 => Ok(Rank::King),
            14 => Ok(Rank::Ace),
            _ => Err(format!("Invalid numerical value for Rank: {}", num).into()),
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

    /// Returns the numerical value of a card's rank.
    pub fn as_num(&self) -> u32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_rank_from_str() {
        assert_eq!(Rank::new_from_str("A").unwrap(), Rank::Ace);
        assert_eq!(Rank::new_from_str("K").unwrap(), Rank::King);
        assert_eq!(Rank::new_from_str("Q").unwrap(), Rank::Queen);
        assert_eq!(Rank::new_from_str("J").unwrap(), Rank::Jack);
        assert_eq!(Rank::new_from_str("T").unwrap(), Rank::Ten);
        assert_eq!(Rank::new_from_str("2").unwrap(), Rank::Two);
    }

    #[test]
    fn invalid_rank_from_str() {
        assert!(Rank::new_from_str("x").is_err());
    }
}
