use crate::hand::Hand;

use super::flush::find_flush;
use super::four_of_a_kind::find_four_of_a_kind;
use super::full_house::find_full_house;
use super::score::{calculate_hand_score, HandRank};
use super::straight::find_straight;
use super::three_of_a_kind::find_three_of_a_kind;

// This function evaluates the given Hand and returns its score as an unsigned 32-bit integer.
pub fn evaluate(hand: &Hand) -> u32 {
    // Create a mutable copy of the hand, so we can sort it without affecting
    // the original. We sort the copied hand by rank in descending order.
    // This is to facilitate the identification of hand ranks.
    let mut hand_desc = hand.clone();
    hand_desc
        .sort_by_rank(false)
        .expect("Failed to sort by rank");

    // Check if the hand contains a flush. This check is performed before
    // checking for a straight flush for performance reasons.
    // If a hand is not a flush, there's no point in checking if it's a straight
    // flush.
    // Moreover, if a hand is a flush but the check for a straight flush fails,
    // we can still utilize the result (that it's a flush) for scoring later.
    let flush_ranks_desc = find_flush(&hand_desc);

    if let Some(flush_ranks) = &flush_ranks_desc {
        let straight_flush_rank_opt = find_straight(&flush_ranks);

        // If straight_flush_rank_opt is Some, meaning a straight flush is found,
        // then calculate and return the hand score for a straight flush.
        if let Some(straight_flush_rank) = straight_flush_rank_opt {
            return calculate_hand_score(vec![straight_flush_rank], HandRank::StraightFlush);
        }
    }

    let ranks_desc = hand_desc.get_ranks();

    // The ranks in descending order without duplicates are calculated here.
    // The reason is that we are trying to reduce the amount of computation needed
    // for evaluating whether a hand is a straight, four of a kind or full house.
    // The number of duplicates in the original hand will inform us whether
    // the checks for a four of a kind or full house are necessary.
    // If the straight check is later needed, the deduplicated ranks are ready for use.
    let mut ranks_desc_no_dup = ranks_desc.clone();
    ranks_desc_no_dup.dedup();
    let num_duplicates = ranks_desc.len() - ranks_desc_no_dup.len();

    if num_duplicates > 2 {
        // Check for a four of a kind in the hand by passing the ranks (in
        // descending order) to the function `find_four_of_a_kind`, which
        // returns an Option.
        if let Some(four_of_a_kind) = find_four_of_a_kind(&ranks_desc) {
            // If a four of a kind is found (i.e., the result is not None),
            // calculate the hand score using the vector result and the
            // FourOfAKind HandRank.
            return calculate_hand_score(four_of_a_kind, HandRank::FourOfAKind);
        }

        // Check for a full house in the hand by passing the ranks (in
        // descending order) to the function `find_full_house`, which also
        // returns an Option.
        if let Some(full_house) = find_full_house(&ranks_desc) {
            // If a full house is found (i.e., the result is not None),
            // calculate the hand score using the vector result and the
            // FullHouse HandRank.
            return calculate_hand_score(full_house, HandRank::FullHouse);
        }
    }

    // If none of the higher hands have been found and there are at least 5
    // cards in the flush then we can finally move the flush_ranks desc out... ;-)
    if let Some(ref flush_ranks_desc) = flush_ranks_desc {
        // Use the first five cards to form a Flush
        let flush_ranks = &flush_ranks_desc[0..5];
        return calculate_hand_score(flush_ranks.to_vec(), HandRank::Flush);
    }

    let straight_rank_opt = find_straight(&ranks_desc_no_dup);
    // If straight_rank_opt is Some, meaning a straight is found,
    // then calculate and return the hand score for a straight flush.
    if let Some(straight_rank) = straight_rank_opt {
        return calculate_hand_score(vec![straight_rank], HandRank::Straight);
    }

    if num_duplicates > 1 {
        let three_of_a_kind_opt = find_three_of_a_kind(&ranks_desc);
        if let Some(three_of_a_kind) = three_of_a_kind_opt {
            return calculate_hand_score(three_of_a_kind, HandRank::ThreeOfAKind);
        }
    }
    return 0;
}
