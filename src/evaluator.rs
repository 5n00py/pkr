use crate::card::Rank;
use crate::hand::Hand;

/// Evaluates the value of a straight in a hand.
///
/// This function is intended to evaluate if the hand forms a straight
/// and calculate its value accordingly. A straight is defined as five cards of
/// sequential rank.
/// Note that Ace can either be the highest card in "A, K, Q, J, T" or the
/// lowest in "5 4 3 2 A". The function accepts a vector of Rank sorted in
/// descending order and returns an integer representing the hand's value if
/// it forms a straight or zero if it does not.
///
/// # Arguments
///
/// * `ranks_desc` - A vector of Rank objects sorted in descending order.
///
/// # Returns
///
/// Returns an u32 value representing the value of the straight, or 0 if the
/// hand is not a straight.
pub fn evaluate_straight(ranks_desc: Vec<Rank>) -> u32 {
    // Ensure we have at least 5 cards for a straight.
    if ranks_desc.len() < 5 {
        return 0;
    }

    // Check for straights.
    let mut highest_straight_card = 0;
    for i in 0..(ranks_desc.len() - 4) {
        // TBD: Skip the following if ranks_desc[i] != ranks_desc[i+4] + 4 as 
        // this is a nessecary condition and might speed up a bit...
        let consecutive_ranks = ranks_desc[i..i + 5]
            .windows(2)
            .all(|window| window[0].num_value() - 1 == window[1].num_value());
        if consecutive_ranks {
            // Update the highest straight card if a straight is found.
            highest_straight_card = ranks_desc[i].num_value();
            // No need to check any further..
            break;
        }
    }

    // Check for ace low straight, if no other straight has been found yet.
    if highest_straight_card == 0 && ranks_desc.contains(&Rank::Ace) {
        let lowest_four_ranks = &ranks_desc[(ranks_desc.len() - 4)..];
        if lowest_four_ranks
            .windows(2)
            .all(|window| window[0].num_value() - 1 == window[1].num_value())
        {
            // 5 is the highest card in an ace low straight.
            highest_straight_card = 5;
        }
    }

    // Return the value of the highest straight found, 
    // or 0 if no straight was found.
    if highest_straight_card > 0 {
        6_000_000 + highest_straight_card
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::Hand;

    #[test]
    fn test_evaluate_straight() {
        let mut hand = Hand::new_from_str("5c 6c 7d 8d 9s Ts Kd").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 6000000 + 10);

        let mut hand = Hand::new_from_str("Ac 2d 3s 4h 5d").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 6000000 + 5);

        let mut hand = Hand::new_from_str("2s 3h 4d 5h Ah").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 6000000 + 5);

        let mut hand = Hand::new_from_str("4d 5h 6s 7d 8h 9d Ts").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 6000000 + 10);

        let mut hand = Hand::new_from_str("Ac 2h 3d 4h 5s 6d 7h").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 6000000 + 7);

        let mut hand = Hand::new_from_str("5d 6s 7h 8d 9h Ts Js").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 6000000 + 11);

        let mut hand = Hand::new_from_str("2h 3d 4s 5h 6d 7h 8d").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 6000000 + 8);

        let mut hand = Hand::new_from_str("9d Ts Js Qh Kd").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 6000000 + 13);

        let mut hand = Hand::new_from_str("9d Ts Ts Qh Kd").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 0);

        let mut hand = Hand::new_from_str("9d Js Js Qh Kd As").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 0);

        let mut hand = Hand::new_from_str("Ad 2s 2c 4h 5d").unwrap();
        hand.sort_by_rank(false).unwrap();
        let ranks_desc = hand.get_ranks();
        assert_eq!(evaluate_straight(ranks_desc), 0);
        
        // TBD: Using Vec<Rank> here, as hand needs 5 cards...
        // let mut hand = Hand::new_from_str("9d Ts Js").unwrap();
        // hand.sort_by_rank(false).unwrap();
        // let ranks_desc = hand.get_ranks();
        // assert_eq!(evaluate_straight(ranks_desc), 0);
    }
}
