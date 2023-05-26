use crate::hand::Hand;

use super::flush::get_flush_ranks;
use super::score::{HandRank, calculate_hand_score};
use super::straight::get_straight_rank_from_desc_nodup;


// This function evaluates the given Hand and returns its score as an unsigned 32-bit integer.
pub fn evaluate(hand: Hand) -> u32 {
    // Create a mutable copy of the hand, so we can sort it without affecting 
    // the original. We sort the copied hand by rank in descending order. 
    // This is to facilitate the identification of hand ranks.
    let mut hand_desc = hand.clone();
    hand_desc.sort_by_rank(false);

    // Check if the hand contains a flush. This check is performed before 
    // checking for a straight flush for performance reasons.
    // If a hand is not a flush, there's no point in checking if it's a straight 
    // flush.
    // Moreover, if a hand is a flush but the check for a straight flush fails, 
    // we can still utilize the result (that it's a flush) for scoring later.
    let flush_ranks_desc = get_flush_ranks(hand_desc);

    if let Some(flush_ranks) = flush_ranks_desc {
        let straight_rank_opt = get_straight_rank_from_desc_nodup(flush_ranks);

        // If straight_rank_opt is Some, meaning a straight is found,
        // then calculate and return the hand score for a straight flush.
        if let Some(straight_rank) = straight_rank_opt {
            return calculate_hand_score(vec![straight_rank], HandRank::StraightFlush);
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Rank;

    #[test]
    fn test_straight_flushes() {
        let hand = Hand::new_from_str("2s As Js Ks Qs 9c Ts").unwrap();
        let desc = "Royal flush";
        let score = evaluate(hand);
        println!("{}, score: {}", desc, score);
        let kicker = Rank::new_from_str("A").unwrap();
        assert_eq!(score, calculate_hand_score(vec![kicker], HandRank::StraightFlush));

        let hand = Hand::new_from_str("2s Kc Js Ks Qs 9s Ts").unwrap();
        let desc = "Straight Flush to King";
        let score = evaluate(hand);
        println!("{}, score: {}", desc, score);
        let kicker = Rank::new_from_str("K").unwrap();
        assert_eq!(score, calculate_hand_score(vec![kicker], HandRank::StraightFlush));

        let hand = Hand::new_from_str("9h 8h Jc Tc Qh Jh Th").unwrap();
        let desc = "Straight Flush to Queen";
        let score = evaluate(hand);
        println!("{}, score: {}", desc, score);
        let kicker = Rank::new_from_str("Q").unwrap();
        assert_eq!(score, calculate_hand_score(vec![kicker], HandRank::StraightFlush));

        let hand = Hand::new_from_str("2s 7s Js 9s 8s 9c Ts").unwrap();
        let desc = "Straight Flush to Jack";
        let score = evaluate(hand);
        println!("{}, score: {}", desc, score);
        let kicker = Rank::new_from_str("J").unwrap();
        assert_eq!(score, calculate_hand_score(vec![kicker], HandRank::StraightFlush));

        let hand = Hand::new_from_str("2d Ad 3d 4d 5d 3c Th").unwrap();
        let desc = "Straight Flush to Five";
        let score = evaluate(hand);
        println!("{}, score: {}", desc, score);
        let kicker = Rank::new_from_str("5").unwrap();
        assert_eq!(score, calculate_hand_score(vec![kicker], HandRank::StraightFlush));
    }
}
