use std::error::Error;

use crate::card::{Card, Rank, Suit};

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
                MIN_CARDS, 
                MAX_CARDS
            ).into());
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
                MIN_CARDS, 
                MAX_CARDS
            ).into());
        }
        let mut cards = Vec::new();
        for s in strings {
            let card =
                Card::new_from_str(s).map_err(|_| format!("Invalid card string: {}", s))?;
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
    let cards = vec![
        Card::new_from_str("3d").unwrap(),
    ];

    let result = Hand::new(cards);
    assert!(result.is_err());
}
