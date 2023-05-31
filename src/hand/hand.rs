use std::error::Error;

use crate::card::{Card, Rank, Suit};

use super::evaluator::evaluator::evaluate;

// The minimum and maximum number of cards a hand can consist of.
const MIN_CARDS: usize = 2;
const MAX_CARDS: usize = 9;

/// Represents a poker hand.
///
/// A poker hand consists of `MIN_CARDS` to `MAX_CARDS` number of cards.
#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Creates a new `Hand` from a vector of cards.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::card::Card;
    /// use pkr::hand::Hand;
    ///
    /// let cards = vec![
    ///     Card::new_from_str("Ah").unwrap(),
    ///     Card::new_from_str("Kh").unwrap(),
    ///     Card::new_from_str("Qh").unwrap(),
    ///     Card::new_from_str("Jh").unwrap(),
    ///     Card::new_from_str("Th").unwrap(),
    /// ];
    ///
    /// let hand = Hand::new(cards).unwrap();
    ///
    /// assert_eq!(hand.get_cards().len(), 5);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the hand does not have between `MIN_CARDS`
    /// and `MAX_CARDS` number of cards.
    pub fn new(cards: Vec<Card>) -> Result<Hand, Box<dyn Error>> {
        let num_cards = cards.len();
        if num_cards < MIN_CARDS || num_cards > MAX_CARDS {
            return Err(format!(
                "A poker hand must have between {} and {} cards.",
                MIN_CARDS, MAX_CARDS
            )
            .into());
        }

        Ok(Hand { cards })
    }

    /// Creates a new `Hand` from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the card identifiers.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::hand::Hand;
    ///
    /// let hand = Hand::new_from_str("As Ks Qs Js Ts").unwrap();
    /// assert_eq!(hand.get_cards().len(), 5);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the string does not represent a valid hand
    /// the hand does not have between `MIN_CARDS` and `MAX_CARDS` number of cards.
    pub fn new_from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let strings: Vec<&str> = s.split_whitespace().collect();
        if strings.len() < MIN_CARDS || strings.len() > MAX_CARDS {
            return Err(format!(
                "A poker hand must have between {} and {} cards.",
                MIN_CARDS, MAX_CARDS
            )
            .into());
        }
        let mut cards = Vec::new();
        for s in strings {
            let card = Card::new_from_str(s).map_err(|_| format!("Invalid card string: {}", s))?;
            cards.push(card);
        }
        Ok(Hand { cards })
    }

    /// Adds a single card to the hand.
    ///
    /// # Arguments
    ///
    /// * `new_card` - A card to be added to the hand.
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if adding the card would result in more than 7 cards in the hand.
    pub fn add_card(&mut self, new_card: Card) -> Result<(), Box<dyn Error>> {
        if self.cards.len() + 1 > MAX_CARDS {
            return Err("Too many cards in the hand.".into());
        }
        self.cards.push(new_card);
        Ok(())
    }

    /// Adds multiple cards to the hand.
    ///
    /// # Arguments
    ///
    /// * `new_cards` - A vector of cards to be added to the hand.
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if adding the cards would result in more than 7 cards in the hand.
    pub fn add_cards(&mut self, new_cards: Vec<Card>) -> Result<(), Box<dyn Error>> {
        if self.cards.len() + new_cards.len() > MAX_CARDS {
            return Err("Too many cards to add.".into());
        }
        for card in new_cards {
            self.cards.push(card);
        }
        Ok(())
    }

    /// Returns a reference to the cards in the hand.
    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    /// Returns the number of cards in the hand.
    pub fn get_count(&self) -> usize {
        self.cards.len()
    }

    pub fn get_score(&self) -> u32 {
        evaluate(self)
    }

    /// Returns the ranks of all cards in the hand, ignoring the suits.
    ///
    /// This can be useful when only the ranks of the cards matter for a certain
    /// operation or comparison, and the suits are irrelevant.
    ///
    /// # Returns
    ///
    /// A Vec of Rank representing the ranks of all cards in the hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::card::{Card, Rank, Suit};
    /// use pkr::hand::Hand;
    ///
    /// let hand = Hand::new(vec![
    ///     Card::new(Rank::Ace, Suit::Heart),
    ///     Card::new(Rank::Two, Suit::Spade),
    ///     Card::new(Rank::Four, Suit::Diamond),
    ///     Card::new(Rank::Five, Suit::Heart),
    ///     Card::new(Rank::Three, Suit::Heart),
    /// ]).unwrap();
    ///
    /// let ranks = hand.get_ranks();
    /// assert_eq!(ranks, vec![Rank::Ace, Rank::Two, Rank::Four, Rank::Five, Rank::Three]);
    /// ```
    pub fn get_ranks(&self) -> Vec<Rank> {
        self.cards.iter().map(|card| card.rank).collect()
    }

    /// Returns a string representation of the `Hand`.
    ///
    /// The string consists of card identifiers separated by spaces. Each card
    /// identifier consists of two characters: the rank and the suit. For
    /// example, the ace of clubs is represented as "Ac".
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::hand::Hand;
    /// use pkr::card::{Card, Rank, Suit};
    ///
    /// let hand = Hand::new(vec![
    ///     Card { rank: Rank::Ace, suit: Suit::Club },
    ///     Card { rank: Rank::King, suit: Suit::Spade },
    ///     Card { rank: Rank::Queen, suit: Suit::Heart },
    ///     Card { rank: Rank::Jack, suit: Suit::Diamond },
    ///     Card { rank: Rank::Ten, suit: Suit::Club },
    /// ]).unwrap();
    ///
    /// assert_eq!(hand.as_str(), "Ac Ks Qh Jd Tc");
    /// ```
    pub fn as_str(&self) -> String {
        self.cards
            .iter()
            .map(|card| card.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Sorts the cards in the hand by suit in ascending order.
    ///
    /// The relative order of cards with the same suit is maintained.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::hand::Hand;
    /// use pkr::card::{Card, Rank, Suit};
    ///
    /// let mut hand = Hand::new(vec![
    ///     Card { rank: Rank::Ace, suit: Suit::Heart },
    ///     Card { rank: Rank::King, suit: Suit::Club },
    ///     Card { rank: Rank::Queen, suit: Suit::Spade },
    ///     Card { rank: Rank::Jack, suit: Suit::Diamond },
    ///     Card { rank: Rank::Ten, suit: Suit::Heart },
    /// ]).unwrap();
    ///
    /// hand.sort_by_suit();
    ///
    /// assert_eq!(hand.as_str(), "Kc Jd Ah Th Qs");
    /// ```
    pub fn sort_by_suit(&mut self) {
        self.cards
            .sort_by(|a, b| a.suit.partial_cmp(&b.suit).unwrap());
    }

    /// Sorts the hand by rank, preserving the original order within each rank.
    ///
    /// # Arguments
    ///
    /// * `ascending` - A boolean indicating if sorting should be in ascending
    ///                 order (true) or descending order (false).
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if the ranks cannot be compared.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::card::{Card, Rank, Suit};
    /// use pkr::hand::Hand;
    ///
    /// let mut hand = Hand::new_from_str("Ah 2s 4d 5h 3h").unwrap();
    /// hand.sort_by_rank(true).unwrap();
    /// assert_eq!(hand.as_str(), "2s 3h 4d 5h Ah");
    ///
    /// hand.sort_by_rank(false).unwrap();
    /// assert_eq!(hand.as_str(), "Ah 5h 4d 3h 2s");
    /// ```
    pub fn sort_by_rank(&mut self, ascending: bool) -> Result<(), Box<dyn Error>> {
        if ascending {
            self.cards
                .sort_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap());
        } else {
            self.cards
                .sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
        }
        Ok(())
    }

    /// Returns all cards in the hand of a given suit.
    ///
    /// # Arguments
    ///
    /// * `suit` - A suit of which the cards are to be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use pkr::hand::Hand;
    /// use pkr::card::{Card, Rank, Suit};
    ///
    /// let hand = Hand::new(vec![
    ///     Card { rank: Rank::Two, suit: Suit::Heart },
    ///     Card { rank: Rank::Three, suit: Suit::Heart },
    ///     Card { rank: Rank::Four, suit: Suit::Spade },
    ///     Card { rank: Rank::Five, suit: Suit::Diamond },
    ///     Card { rank: Rank::Six, suit: Suit::Heart },
    /// ]).unwrap();
    ///
    /// let hearts = hand.cards_of_suit(Suit::Heart);
    /// assert_eq!(hearts.len(), 3);
    /// ```
    pub fn cards_of_suit(&self, suit: Suit) -> Vec<Card> {
        self.cards
            .iter()
            .filter(|&card| card.suit == suit)
            .cloned()
            .collect()
    }
}

#[test]
fn test_create_hand() {
    let cards = vec![
        Card::new_from_str("2h").unwrap(),
        Card::new_from_str("3d").unwrap(),
        Card::new_from_str("4s").unwrap(),
        Card::new_from_str("5c").unwrap(),
        Card::new_from_str("6h").unwrap(),
        Card::new_from_str("7d").unwrap(),
        Card::new_from_str("8s").unwrap(),
    ];

    let hand = Hand::new(cards);

    assert!(hand.is_ok());

    let hand = hand.unwrap();
    assert_eq!(hand.get_cards().len(), 7)
}

#[test]
fn test_create_hand_with_wrong_number_of_cards() {
    let cards = vec![Card::new_from_str("3d").unwrap()];

    let result = Hand::new(cards);
    assert!(result.is_err());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_flushes() {
        let hand = Hand::new_from_str("2s As Js Ks Qs 9c Ts").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 14);

        let hand = Hand::new_from_str("2s Kc Js Ks Qs 9s Ts").unwrap();
        let score = hand.get_score();

        assert_eq!(score, 8_000_000 + 13);

        let hand = Hand::new_from_str("9h 8h Jc Tc Qh Jh Th").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 12);

        let hand = Hand::new_from_str("2s 7s Js 9s 8s 9c Ts").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 11);

        let hand = Hand::new_from_str("9d 8d Td 7d 6d 3c Th Kh Qd").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 10);

        let hand = Hand::new_from_str("9d 8d 5d 6d 7d").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 9);

        let hand = Hand::new_from_str("4c 5c 6c 7c 8c 3c 2c").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 8);

        let hand = Hand::new_from_str("7d 7c 7s 6d 5d 3d 4d").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 7);

        let hand = Hand::new_from_str("6d 5d 4d 3d 2d Ad").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 6);

        let hand = Hand::new_from_str("2d Ad 3d 4d 5d 3c Th").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 8_000_000 + 5);
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = Hand::new_from_str("As Ac Ad Ah Ts 9c Qs").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 7_000_000 + (14 << 4) + 12);

        let hand = Hand::new_from_str("As Ac Ad Ah").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 7_000_000 + 14);

        let hand = Hand::new_from_str("9c Ks Kc Kd Kh Ts 2s").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 7_000_000 + (13 << 4) + 10);

        let hand = Hand::new_from_str("Qs Qc Qd Qh 8s 9c 9s").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 7_000_000 + (12 << 4) + 9);

        let hand = Hand::new_from_str("2s 2c 2d 2h As 9c Qs").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 7_000_000 + (2 << 4) + 14);
    }

    #[test]
    fn test_full_house() {
        let hand = Hand::new_from_str("As Ac Ad Kh Ts Kc Qs").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 6_000_000 + (14 << 4) + 13);

        let hand = Hand::new_from_str("Ks Qc Kd Kh Qd").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 6_000_000 + (13 << 4) + 12);

        let hand = Hand::new_from_str("Tc 9s 9c Td 9h Ts 2s").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 6_000_000 + (10 << 4) + 9);

        let hand = Hand::new_from_str("4s 4c 4d 5h 5s 9c 9s").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 6_000_000 + (4 << 4) + 9);

        let hand = Hand::new_from_str("2s 2c 2d 3h As 3c Qs").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 6_000_000 + (2 << 4) + 3);
    }

    #[test]
    fn test_flush() {
        let hand = Hand::new_from_str("As Ks Qs Js 9s 8s 7s").unwrap();
        let score = hand.get_score();
        assert_eq!(
            score,
            5_000_000 + (14 << 16) + (13 << 12) + (12 << 8) + (11 << 4) + 9
        );

        // Check corner case vs. lowest full house
        let hand = Hand::new_from_str("2s 2c 2d 3h As 3c Qs").unwrap();
        let score_low_fh = hand.get_score();
        assert!(score < score_low_fh);

        let hand = Hand::new_from_str("Ks Kd Qs Js 9s 9d 7s").unwrap();
        let score = hand.get_score();
        assert_eq!(
            score,
            5_000_000 + (13 << 16) + (12 << 12) + (11 << 8) + (9 << 4) + 7
        );

        let hand = Hand::new_from_str("Qs Js 9s 8s 7s").unwrap();
        let score = hand.get_score();
        assert_eq!(
            score,
            5_000_000 + (12 << 16) + (11 << 12) + (9 << 8) + (8 << 4) + 7
        );

        let hand = Hand::new_from_str("7s Kd Qd 4s 5s 3s 2s").unwrap();
        let score = hand.get_score();
        assert_eq!(
            score,
            5_000_000 + (7 << 16) + (5 << 12) + (4 << 8) + (3 << 4) + 2
        );
    }

    #[test]
    fn test_straight() {
        let hand = Hand::new_from_str("2d Ac Js Ks Qs 9c Ts").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 14);

        let hand = Hand::new_from_str("2s Kc Jh Kd Qs 9s Ts").unwrap();
        let score = hand.get_score();

        assert_eq!(score, 4_000_000 + 13);

        let hand = Hand::new_from_str("9c 8h Jc Tc Qs Jh Th").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 12);

        let hand = Hand::new_from_str("2c 7c Js 9s 8h 9c Ts").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 11);

        let hand = Hand::new_from_str("9h 8d Ts 7d 6c 3c Th Kh Qd").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 10);

        let hand = Hand::new_from_str("9c 8h 5d 6d 7d").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 9);

        let hand = Hand::new_from_str("4c 5d 6c 7h 8c 3d 2c").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 8);

        let hand = Hand::new_from_str("7d 7c 7s 6d 5c 3d 4d").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 7);

        let hand = Hand::new_from_str("6d 5d 4d 3c 2d Ac").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 6);

        let hand = Hand::new_from_str("2d Ac 3d 4d 5d 3c Th").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 4_000_000 + 5);
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = Hand::new_from_str("2s Ac Ad Ah Ts 9c Qs").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 3_000_000 + (14 << 8) + (12 << 4) + 10);

        let hand = Hand::new_from_str("As Ac Ad 2h").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 3_000_000 + (14 << 4) + 2);

        let hand = Hand::new_from_str("As Ac Ad").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 3_000_000 + 14);

        let hand = Hand::new_from_str("9c Ks Kc Kd Ah Ts 2s").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 3_000_000 + (13 << 8) + (14 << 4) + 10);

        let hand = Hand::new_from_str("9c 3s 2c 2d Kh Ts 2s").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 3_000_000 + (2 << 8) + (13 << 4) + 10);

        let hand = Hand::new_from_str("2s 2c 2d").unwrap();
        let score = hand.get_score();
        assert_eq!(score, 3_000_000 + 2);
    }
}
