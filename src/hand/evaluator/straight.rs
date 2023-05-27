use crate::card::Rank;

/// Evaluates if a descending ordered and duplicate-free rank vector contains a
/// straight.
///
/// In poker, a straight is a hand that contains five cards of sequential rank,
/// not all of the same suit.
///
/// A special case, Ace low straight (Five, Four, Three, Two, Ace), is also
/// handled by this function.
///
/// # Arguments
///
/// * `ranks_desc` - A vector of `Rank` values sorted in descending order and 
///   without duplicates.
///
/// # Returns
///
/// * An `Option<Rank>` which is `Some(Rank)` of the highest card in the 
///   straight if a straight is found, or `None` if no straight is found.
pub fn find_straight(mut ranks_desc_nodup: Vec<Rank>) -> Option<Rank> {
    if let Some(&Rank::Ace) = ranks_desc_nodup.first() {
        ranks_desc_nodup.push(Rank::AceLow);
    }
    for i in 0..=(ranks_desc_nodup.len() - 5) {
        if ranks_desc_nodup[i] as u8 == ranks_desc_nodup[i + 4] as u8 + 4 {
            return Some(ranks_desc_nodup[i]);
        }
    }

    return None;
}
