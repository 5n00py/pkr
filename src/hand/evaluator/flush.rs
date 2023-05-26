/// Evaluates the rank of a poker hand for a Flush.
///
/// This function determines whether a given hand is a Flush and calculates 
/// its score. The score is a combination of the HandRank value for a Flush and 
/// the rank scores of the top five cards in the flush suit.
///
/// Note that this function is not exclusive in its check; a hand evaluated by 
/// this function could still potentially satisfy a higher rank like a Straight 
/// Flush.
///
/// # Arguments
///
/// * `hand` - A hand of cards.
///
/// # Returns
///
/// * A `u32` score if the hand conatins a Flush or 0 if it does not contain a 
/// Flush.
fn evaluate_flush(hand: Hand) -> u32 {
    let flush_ranks = get_flush_ranks_desc(hand);
    
    if flush_ranks.is_empty() {
        return 0;
    } else {
        let flush_ranks_five = &flush_ranks[0..5];
        let score = calculate_rank_score(flush_ranks_five.to_vec());
        return score + HandRank::Flush as u32;
    }
}

/// Gets the ranks of the flush cards in a `hand` in descending order if a flush 
/// exists or returns an empty vector if a hand does not contain a flush.
///
/// # Arguments
///
/// * `hand` - A hand of cards.
///
/// # Returns
///
/// * The ranks of the flush cards in descending order if a flush exists or an 
///   empty vector if not.
fn get_flush_ranks_desc(hand: Hand) -> Vec<Rank> {
    for suit in Suit::iter() {
        let mut flush_cards = hand.cards_of_suit(suit);
        if flush_cards.len() >= 5 {
            flush_cards.sort_by(|a, b| b.rank.cmp(&a.rank));  // sort flush_cards by rank in descending order
            return flush_cards.into_iter().map(|card| card.rank).collect();
        }
    }
    Vec::new()  // return empty Vec if no flush
}


