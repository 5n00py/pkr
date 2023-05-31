use crate::card::Rank;

/// Finds in a descending ordered and duplicate-free rank vector straight or
/// returns None.
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
pub fn find_straight(ranks_desc_nodup: &Vec<Rank>) -> Option<Rank> {
    let ranks_len = ranks_desc_nodup.len();

    if ranks_len < 5 {
        return None;
    }

    for i in 0..=(ranks_len - 5) {
        if ranks_desc_nodup[i] as u8 == ranks_desc_nodup[i + 4] as u8 + 4 {
            return Some(ranks_desc_nodup[i]);
        }
    }

    if ranks_desc_nodup[0] == Rank::Ace
        && ranks_desc_nodup[ranks_len - 1] == Rank::Two
        && ranks_desc_nodup[ranks_len - 2] == Rank::Three
        && ranks_desc_nodup[ranks_len - 3] == Rank::Four
        && ranks_desc_nodup[ranks_len - 4] == Rank::Five
    {
        return Some(Rank::Five);
    }

    return None;
}
