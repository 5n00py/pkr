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
    Two,
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
