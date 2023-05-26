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
    RoyalFlush = 9_000_000,
}

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
/// priority for scoring. For example for a straight "A, K, Q, J, T", the Ace
/// should come before the King in the ranks list, etc. Note that this function 
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
pub fn calculate_rank_score(ranks: Vec<Rank>) -> u32 {
    if ranks.is_empty() {
        return 0;
    }

    let mut score: u32 = 0;

    for rank in ranks {
        score = (score << 4) | (rank as u32);
    }

    score
}


