use strum::IntoEnumIterator;

use crate::card::{Rank, Suit };
use crate::hand::Hand;

/// Gets the ranks of the flush cards in a `hand` in the order they were passed 
/// if a flush exists or returns None if a hand does not contain a flush.
///
/// # Arguments
///
/// * `hand` - A hand of cards.
///
/// # Returns
///
/// * The ranks of the flush cards in the order they were passed if a flush exists or None
///   if not.
fn get_flush_ranks(hand: Hand) -> Option<Vec<Rank>> {
    for suit in Suit::iter() {
        let flush_cards = hand.cards_of_suit(suit);
        if flush_cards.len() >= 5 {
            return Some(flush_cards.into_iter().map(|card| card.rank).collect());
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Suit, Rank};
    use crate::hand::Hand;

    #[test]
    fn test_get_flush_ranks_with_flush() {
        let hand = Hand::new_from_str("As Ks Qs Js Ts").unwrap();
        let result = get_flush_ranks(hand).unwrap();
        assert_eq!(result, vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten]);

        let hand = Hand::new_from_str("Ks Kd Qc Js Ts 9s As").unwrap();
        let result = get_flush_ranks(hand).unwrap();
        assert_eq!(result, vec![Rank::King, Rank::Jack, Rank::Ten, Rank::Nine, Rank::Ace]);
    }

    #[test]
    fn test_get_flush_ranks_without_flush() {
        let hand = Hand::new_from_str("As Kd Qs Jd Tc").unwrap();
        let result = get_flush_ranks(hand);
        assert!(result.is_none());
    }
}

