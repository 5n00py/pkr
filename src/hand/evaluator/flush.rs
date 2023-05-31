use strum::IntoEnumIterator;

use crate::card::{Rank, Suit};
use crate::hand::Hand;

/// Finds the ranks of the flush cards in a `hand` in the order they were passed
/// if a flush exists or returns None if a hand does not contain a flush.
///
/// # Arguments
///
/// * `hand` - A hand of cards.
///
/// # Returns
///
/// * The ranks of the flush cards in the order they were passed if a flush
/// exists or None if not.
pub fn find_flush(hand: &Hand) -> Option<Vec<Rank>> {
    for suit in Suit::iter() {
        let flush_cards = hand.cards_of_suit(suit);
        if flush_cards.len() >= 5 {
            return Some(flush_cards.into_iter().map(|card| card.rank).collect());
        }
    }
    None
}
