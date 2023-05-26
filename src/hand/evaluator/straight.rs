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


