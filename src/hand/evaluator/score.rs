use crate::card::Rank;

/// An enumeration representing the rank of a poker hand.
///
/// Each variant corresponds to a different type of hand in poker. The numerical
/// values assigned to each variant represent their relative strength, with a
/// higher number indicating a stronger hand. These values can be used to compare
/// hands and determine the winner in a game of poker.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard = 0,
    OnePair = 1_000_000,
    TwoPair = 2_000_000,
    ThreeOfAKind = 3_000_000,
    Straight = 4_000_000,
    Flush = 5_000_000,
    FullHouse = 6_000_000,
    FourOfAKind = 7_000_000,
    StraightFlush = 8_000_000,
}

/// Calculates the final score for a hand of cards.
///
/// This score is computed by adding the value of the hand's rank (represented
/// by a value from the `HandRank` enum) to the score of the card ranks (computed
/// by the `calculate_rank_score` function).
///
/// This scoring system is able to account for all possible poker hands, as
/// it allows for differentiating between hands with different combinations
/// of ranks and for identifying the relative value of hands with the same
/// combination of ranks.
///
/// # Arguments
///
/// * `ranks` - A vector of card ranks, not necessarily in order.
/// * `hand_rank` - The rank of the hand, as a value from the `HandRank` enum.
///
/// # Returns
///
/// * The final score of the hand as an u32 integer.
pub fn calculate_hand_score(ranks: Vec<Rank>, hand_rank: HandRank) -> u32 {
    let rank_score = calculate_rank_score(ranks);
    let hand_score = hand_rank as u32;

    hand_score + rank_score
}

/// Calculates the score from a list of card ranks.
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
/// priority for scoring. For example for a full house "2, 2, 2, A, A", the Ace
/// should come before the 2 in the ranks list, etc. Note that this function
/// does not sort the ranks before calculating the score.
///
/// # Arguments
///
/// * `ranks` - A vector of card ranks, not necessarily in order.
///
/// # Returns
///
/// * The score of the ranks as an u32 integer.
///   If the list of ranks is empty, returns 0.
fn calculate_rank_score(ranks: Vec<Rank>) -> u32 {
    // If ranks vector contains less than 5 ranks, resize it to 5
    // filling with Rank::Two (which corresponds to zero value as per your requirement).
    if ranks.is_empty() {
        return 0;
    }

    let mut score: u32 = 0;

    // Evaluate only the first five ranks
    for rank in ranks.into_iter() {
        score = (score << 4) | (rank as u32);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_rank_score() {
        // check non-empty list of ranks
        let ranks = vec![Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::King];
        assert_eq!(calculate_rank_score(ranks), 978669);

        let ranks = vec![Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::Queen];
        assert_eq!(calculate_rank_score(ranks), 978668);

        let ranks = vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten];
        assert_eq!(calculate_rank_score(ranks), 974010);

        let ranks = vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine];
        assert_eq!(calculate_rank_score(ranks), 974009);

        let score = calculate_rank_score(vec![Rank::Ace, Rank::King, Rank::Queen]);
        assert_eq!(score, 0b1110_1101_1100);

        let score = calculate_rank_score(vec![Rank::Two, Rank::Three, Rank::Four]);
        assert_eq!(score, 0b0010_0011_0100);

        let score = calculate_rank_score(vec![Rank::Ten, Rank::Nine, Rank::Eight]);
        assert_eq!(score, 0b1010_1001_1000);

        // check ranks out of order
        let score = calculate_rank_score(vec![Rank::Two, Rank::Ace, Rank::Three]);
        assert_eq!(score, 0b0010_1110_0011);

        // check with duplicates
        let score = calculate_rank_score(vec![Rank::Ace, Rank::Ace, Rank::King]);
        assert_eq!(score, 0b1110_1110_1101);
    }

    #[test]
    fn test_calculate_rank_score_empty() {
        // check empty list of ranks
        let result = calculate_rank_score(vec![]);
        assert_eq!(result, 0);
    }
}
