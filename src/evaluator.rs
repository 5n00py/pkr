use strum::IntoEnumIterator;

use crate::card::{ Rank, Suit };
use crate::hand::Hand;

/// Gets the ranks of the flush cards in a `hand` if a flush exists, or None if
/// a hand does not contain a flush.
///
/// # Arguments
///
/// * `hand` - A hand of cards.
///
/// # Returns
///
/// * `Option<Vec<Rank>>` - The ranks of the flush cards if a flush exists, 
///                         or None if not.
fn get_flush_ranks(hand: Hand) -> Option<Vec<Rank>> {
    for suit in Suit::iter() {
        let flush_cards = hand.cards_of_suit(suit);
        if flush_cards.len() >= 5 {
            return Some(flush_cards.into_iter().map(|card| card.rank).collect());
        }
    }
    None
}

/// Checks if a vector of `Rank` forms an Ace-low straight
///
/// # Arguments
/// * `ranks` - A vector of ranks 
/// 
/// # Returns
/// * A boolean indicating whether the ranks form an Ace-low straight.
fn is_ace_low_straight(ranks: &Vec<Rank>) -> bool {
    if ranks.len() < 5 {
        return false;
    }

    // Check for A, 2, 3, 4, 5 in ranks
    let ace_low_straight_ranks = [Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two];
    ace_low_straight_ranks.iter().all(|rank| ranks.contains(rank))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_flush_ranks_with_flush() {
        let mut hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Qs").unwrap();
        hand.sort_by_rank(false).unwrap();
        let flush_ranks = get_flush_ranks(hand).unwrap();
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Queen, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

        let mut hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Qc").unwrap();
        hand.sort_by_rank(false).unwrap();
        let flush_ranks = get_flush_ranks(hand).unwrap();
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

        let mut hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Kc").unwrap();
        hand.sort_by_rank(false).unwrap();
        let flush_ranks = get_flush_ranks(hand).unwrap();
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

    }

    #[test]
    fn test_get_flush_ranks_without_flush() {
        let hand = Hand::new_from_str("2s 4s 6s 8d Td Kd Qs").unwrap();
        assert!(get_flush_ranks(hand).is_none());
    }

    #[test]
    fn test_is_ace_low_straight_true() {
        let hand = Hand::new_from_str("Ac 2h 3d 4s 5c").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 5h 2d 4s 3c").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 2h 3d 4s 5c 6s 7d").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 5h 2d 4s 3c Ad 3s").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));

    }

    #[test]
    fn test_is_ace_low_straight_false() {
        let hand = Hand::new_from_str("2c 3h 4d 5s 6c").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(!is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 2h 3d 4s").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(!is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 2h 3d 4s 5c 6s").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));
    }
}
