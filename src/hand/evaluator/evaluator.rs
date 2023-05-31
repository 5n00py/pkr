use crate::card::Rank;
use crate::hand::Hand;

use super::flush::find_flush;
use super::four_of_a_kind::find_four_of_a_kind;
use super::full_house::find_full_house;
use super::pair::find_pair;
use super::score::{calculate_hand_score, HandRank};
use super::straight::find_straight;
use super::three_of_a_kind::find_three_of_a_kind;
use super::two_pair::find_two_pair;

/// Evaluates a given poker hand and returns its score as a u32.
///
/// # Arguments
///
/// * `hand` - A reference to a Hand object.
///
/// # Returns
///
/// * `u32` - An unsigned 32-bit integer representing the score of the hand.
///
/// # Remarks
///
/// This function evaluates the given Hand object based on the rules of Poker.
/// The hand is evaluated in the following order:
/// 1. Straight Flush
/// 2. Four of a Kind
/// 3. Full House
/// 4. Flush
/// 5. Straight
/// 6. Three of a Kind
/// 7. Two Pair
/// 8. One Pair
/// 9. High Card
///
/// The function is designed to early return as soon as a matching hand rank
/// is found. This minimizes unnecessary computation.
/// If no match is found for the above hand ranks, the hand is evaluated as a
/// high card hand.
///
/// # Panics
///
/// This function may panic in two scenarios:
/// 1. Failed to sort the hand by rank.
/// 2. In the case where it's expecting a paired hand (i.e., One Pair,
/// Two Pair, Three of a Kind), but none is found.
pub fn evaluate(hand: &Hand) -> u32 {
    let mut hand_desc = hand.clone();
    hand_desc
        .sort_by_rank(false)
        .expect("Failed to sort by rank");

    // Check for a flush before a straight flush for performance reasons.
    let flush_ranks_desc = find_flush(&hand_desc);

    // If a straight flush is found, calculate and return the score.
    if let Some(flush_ranks) = &flush_ranks_desc {
        if let Some(straight_flush_rank) = find_straight(&flush_ranks) {
            return calculate_hand_score(vec![straight_flush_rank], HandRank::StraightFlush);
        }
    }

    let ranks_desc = hand_desc.get_ranks();
    let mut ranks_desc_no_dup = ranks_desc.clone();
    ranks_desc_no_dup.dedup();
    let num_duplicates = ranks_desc.len() - ranks_desc_no_dup.len();

    // Check for four of a kind or full house.
    if num_duplicates > 2 {
        if let Some(four_of_a_kind) = find_four_of_a_kind(&ranks_desc) {
            return calculate_hand_score(four_of_a_kind, HandRank::FourOfAKind);
        }
        if let Some(full_house) = find_full_house(&ranks_desc) {
            return calculate_hand_score(full_house, HandRank::FullHouse);
        }
    }

    // Check for a flush.
    if let Some(ref flush_ranks_desc) = flush_ranks_desc {
        let flush_ranks = &flush_ranks_desc[0..5];
        return calculate_hand_score(flush_ranks.to_vec(), HandRank::Flush);
    }

    // Check for a straight.
    if let Some(straight_rank) = find_straight(&ranks_desc_no_dup) {
        return calculate_hand_score(vec![straight_rank], HandRank::Straight);
    }

    // Check for three of a kind, two pair, or one pair.
    if num_duplicates > 1 {
        if let Some(three_of_a_kind) = find_three_of_a_kind(&ranks_desc) {
            return calculate_hand_score(three_of_a_kind, HandRank::ThreeOfAKind);
        }
        if let Some(two_pair) = find_two_pair(&ranks_desc) {
            return calculate_hand_score(two_pair, HandRank::TwoPair);
        }
        panic!("No paired hand found but expected.");
    }

    if num_duplicates > 0 {
        if let Some(pair) = find_pair(&ranks_desc) {
            return calculate_hand_score(pair, HandRank::OnePair);
        }
        panic!("No paired hand found but expected.");
    }

    // Return score for high cards.
    let high_cards: Vec<Rank>;
    if ranks_desc.len() < 5 {
        high_cards = ranks_desc.clone();
    } else {
        high_cards = ranks_desc[0..5].to_vec();
    }
    return calculate_hand_score(high_cards.to_vec(), HandRank::HighCard);
}
