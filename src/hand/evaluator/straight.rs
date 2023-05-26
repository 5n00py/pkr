use crate::card::Rank;

/// Checks if a vector of `Rank` forms an Ace-low straight.
///
/// This function checks for the specific sequence of A, 2, 3, 4, 5 in the given 
/// ranks. It is designed to detect the specific case of an Ace-low straight 
/// (also known as a wheel). Note that if the ranks form a higher straight that 
/// includes an Ace (such as A, 2, 3, 4, 5, 6, 7), this function will still 
/// return true, as it also forms an Ace-low straight.
///
/// TBD: If you need to check for the highest straight in a hand, consider using 
/// this function in combination with another function that checks for 
/// straights more generally.
///
/// # Arguments
///
/// * `ranks` - A vector of ranks 
/// 
/// # Returns
///
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
    use crate::hand::Hand;
    
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
