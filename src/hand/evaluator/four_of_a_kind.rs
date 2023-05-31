use std::option::Option;

use crate::card::Rank;

/// Find "Four of a Kind" in a hand of poker cards.
///
/// The function takes a vector of Rank sorted in descending order.
/// It checks for the occurrence of four cards of the same rank.
/// If the hand has less than five cards, it returns None, except when the hand consists of
/// four cards of the same rank. In this case, we add a placeholder (Ace_low) to serve as a kicker.
/// When a "four of a kind" is found, it returns a vector consisting of two ranks:
/// The first represents the value of the four of a kind,
/// and the second represents the highest card that is not part of the four of a kind (kicker).
///
/// # Arguments
///
/// * `mut ranks` - A mutable vector of Rank representing the ranks of a hand of cards in descending order.
pub fn find_four_of_a_kind(ranks: &Vec<Rank>) -> Option<Vec<Rank>> {
    let ranks_len = ranks.len();

    if ranks_len < 4 {
        return None;
    }

    for i in 0..(ranks.len() - 3) {
        if ranks[i] == ranks[i + 1] && ranks[i + 1] == ranks[i + 2] && ranks[i + 2] == ranks[i + 3]
        {
            let mut four_of_a_kind: Vec<Rank> = Vec::new();
            four_of_a_kind.push(ranks[i]);

            if ranks_len > 4 {
                // Find the highest card that is not part of the four of a kind
                let kicker = ranks.iter().filter(|&&rank| rank != ranks[i]).max();
                match kicker {
                    Some(k) => four_of_a_kind.push(*k),
                    None => return None,
                }
            }
            return Some(four_of_a_kind);
        }
    }

    None
}
