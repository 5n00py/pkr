use std::error::Error;

use strum::IntoEnumIterator;

use crate::card::{ Rank, Suit };
use crate::hand::Hand;


/// Calculates the score from a set of card ranks.
///
/// This score is computed by shifting a 32-bit integer left by 4 bits for each 
/// rank in the list, and then adding the numeric value of the rank to the 
/// integer.
///
/// This process effectively converts a list of ranks into a single number that 
/// represents the order and frequency of ranks in the list.
///
/// This scoring system is able to account for all possible poker hands, as
/// it allows for differentiating between hands with different combinations
/// of ranks and for identifying the relative value of hands with the same
/// combination of ranks.
///
/// It is assumed that ranks are passed in an order that represents the desired
/// priority for scoring. For example, in most poker games, an Ace is considered
/// higher than a King, so for a hand of [Ace, King, Queen, Jack, Ten], the Ace
/// should come before the King in the ranks list. Note that this function does
/// not sort the ranks before calculating the score.
///
/// # Arguments
///
/// * `ranks` - A vector of card ranks, not necessarily in order.
///
/// # Returns
///
/// * A `Result<u32, Box<dyn Error>>` - The score of the ranks as an u32 integer
/// or a Box<dyn Error> if the list of ranks is empty.
pub fn calculate_rank_score(ranks: Vec<Rank>) -> Result<u32, Box<dyn Error>> {
    if ranks.is_empty() {
        return Err("Ranks must not be empty".into());
    }

    let mut score: u32 = 0;

    for rank in ranks {
        // shift the score left by 4 bits to make room for the next card,
        // and add the rank of the card.
        // This works because rank values are 14 or less, so they fit in 4 bits.
        score = (score << 4) | (rank as u32);
    }

    Ok(score)
}

/// Gets the ranks of the flush cards in a `hand` in descending order if a flush 
/// exists or returns None if a hand does not contain a flush.
///
/// # Arguments
///
/// * `hand` - A hand of cards.
///
/// # Returns
///
/// * The ranks of the flush cards in descending order if a flush exists or None 
/// if not.
fn get_flush_ranks_desc(hand: Hand) -> Option<Vec<Rank>> {
    for suit in Suit::iter() {
        let mut flush_cards = hand.cards_of_suit(suit);
        if flush_cards.len() >= 5 {
            flush_cards.sort_by(|a, b| b.rank.cmp(&a.rank));  // sort flush_cards by rank in descending order
            return Some(flush_cards.into_iter().map(|card| card.rank).collect());
        }
    }
    None
}

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
    
    #[test]
    fn test_calculate_rank_score() {
        // check non-empty list of ranks
        let ranks = vec![Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::King];
        assert_eq!(calculate_rank_score(ranks).unwrap(), 978669);
        
        let ranks = vec![Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::Queen];
        assert_eq!(calculate_rank_score(ranks).unwrap(), 978668);

        let ranks = vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten];
        assert_eq!(calculate_rank_score(ranks).unwrap(), 974010);

        let ranks = vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine];
        assert_eq!(calculate_rank_score(ranks).unwrap(), 974009);

        let score = calculate_rank_score(vec![Rank::Ace, Rank::King, Rank::Queen]).unwrap();
        assert_eq!(score, 0b1110_1101_1100);

        let score = calculate_rank_score(vec![Rank::Two, Rank::Three, Rank::Four]).unwrap();
        assert_eq!(score, 0b0010_0011_0100);

        let score = calculate_rank_score(vec![Rank::Ten, Rank::Nine, Rank::Eight]).unwrap();
        assert_eq!(score, 0b1010_1001_1000);

        // check single rank
        let score = calculate_rank_score(vec![Rank::Ace]).unwrap();
        assert_eq!(score, 14);
        
        // check ranks out of order
        let score = calculate_rank_score(vec![Rank::Two, Rank::Ace, Rank::Three]).unwrap();
        assert_eq!(score, 0b0010_1110_0011);
        
        // check with duplicates
        let score = calculate_rank_score(vec![Rank::Ace, Rank::Ace, Rank::King]).unwrap();
        assert_eq!(score, 0b1110_1110_1101);
    }

    #[test]
    fn test_calculate_rank_score_empty() {
        // check empty list of ranks
        let result = calculate_rank_score(vec![]);
        println!("Result: {:?}", result);
        assert_eq!(result.unwrap_err().to_string(), "Ranks must not be empty");
    }

    #[test]
    fn test_get_flush_ranks_with_flush() {
        let hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Qs").unwrap();
        let flush_ranks = get_flush_ranks_desc(hand).unwrap();
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Queen, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

        let hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Qc").unwrap();
        let flush_ranks = get_flush_ranks_desc(hand).unwrap();
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

        let hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Kc").unwrap();
        let flush_ranks = get_flush_ranks_desc(hand).unwrap();
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

    }

    #[test]
    fn test_get_flush_ranks_without_flush() {
        let hand = Hand::new_from_str("2s 4s 6s 8d Td Kd Qs").unwrap();
        assert!(get_flush_ranks_desc(hand).is_none());
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
