use crate::card::Card;

/// Represents a poker hand.
///
/// A poker hand consists of 5 to 7 cards.
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
    ///     Card::new_from_string("Ah").unwrap(),
    ///     Card::new_from_string("Kh").unwrap(),
    ///     Card::new_from_string("Qh").unwrap(),
    ///     Card::new_from_string("Jh").unwrap(),
    ///     Card::new_from_string("Th").unwrap(),
    /// ];
    ///
    /// let hand = Hand::new(cards).unwrap();
    ///
    /// assert_eq!(hand.get_cards().len(), 5);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `std::error::Error` if the hand does not have between 5 and 7 cards.
    pub fn new(cards: Vec<Card>) -> Result<Hand, Box<dyn std::error::Error>> {
        let num_cards = cards.len();
        if num_cards < 5 || num_cards > 7 {
            return Err("A poker hand must have between 5 and 7 cards.".into());
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
    /// let hand = Hand::new_from_string("As Ks Qs Js Ts").unwrap();
    /// assert_eq!(hand.get_cards().len(), 5);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `std::error::Error` if the string does not represent a valid hand.
    pub fn new_from_string(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let strings: Vec<&str> = s.split_whitespace().collect();
        if strings.len() < 5 || strings.len() > 7 {
            return Err("A hand poker hand must have between 5 and 7 cards.".into());
        }

        let mut cards = Vec::new();
        for s in strings {
            let card =
                Card::new_from_string(s).map_err(|_| format!("Invalid card string: {}", s))?;
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
    /// Returns a `std::error::Error` if adding the card would result in more than 7 cards in the hand.
    pub fn add_card(&mut self, new_card: Card) -> Result<(), Box<dyn std::error::Error>> {
        if self.cards.len() + 1 > 7 {
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
    /// Returns a `std::error::Error` if adding the cards would result in more than 7 cards in the hand.
    pub fn add_cards(&mut self, new_cards: Vec<Card>) -> Result<(), Box<dyn std::error::Error>> {
        if self.cards.len() + new_cards.len() > 7 {
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
}

#[test]
fn test_create_hand() {
    let cards = vec![
        Card::new_from_string("2h").unwrap(),
        Card::new_from_string("3d").unwrap(),
        Card::new_from_string("4s").unwrap(),
        Card::new_from_string("5c").unwrap(),
        Card::new_from_string("6h").unwrap(),
        Card::new_from_string("7d").unwrap(),
        Card::new_from_string("8s").unwrap(),
    ];

    let hand = Hand::new(cards);

    assert!(hand.is_ok());

    let hand = hand.unwrap();
    assert_eq!(hand.get_cards().len(), 7)
}

#[test]
fn test_create_hand_with_wrong_number_of_cards() {
    let cards = vec![
        Card::new_from_string("2h").unwrap(),
        Card::new_from_string("3d").unwrap(),
    ];

    let result = Hand::new(cards);
    assert!(result.is_err());
}
