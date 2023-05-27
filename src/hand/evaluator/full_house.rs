use crate::card::Rank;

/// Checks if a given descending sorted `Vec<Rank>` forms a full house.
///
/// A full house in poker is a hand consisting of a three-of-a-kind and a pair.
/// This function takes as input a vector of `Rank`, which should be sorted in
/// descending order. If the length of `ranks_desc` is less than 5, it
/// immediately returns `None`.
///
/// If a full house is found, it returns a `Vec<Rank>` where the first rank is
/// that of the three-of-a-kind, and the second rank is that of the pair
///
/// # Arguments
///
/// * `ranks_desc` - A vector of ranks sorted in descending order.
///
/// # Returns
///
/// * An `Option<Vec<Rank>>` which is `Some(Vec<Rank>)` containing the rank of
/// the three of a kind and the rank of the pair if a full house is found, or
/// `None` if no full house is found.

pub fn find_full_house(ranks_desc: &Vec<Rank>) -> Option<Vec<Rank>> {
    if ranks_desc.len() < 5 {
        return None;
    }

    let mut three_of_a_kind_rank = None;

    // first pass, looking for three of a kind
    for i in 0..ranks_desc.len() - 2 {
        if ranks_desc[i] == ranks_desc[i + 2] {
            three_of_a_kind_rank = Some(ranks_desc[i]);
            break;
        }
    }

    // if no three of a kind was found, return None
    if three_of_a_kind_rank.is_none() {
        return None;
    }

    // second pass, looking for a pair that is not the same as the three of a kind
    for i in 0..ranks_desc.len() - 1 {
        if ranks_desc[i] == ranks_desc[i + 1] && ranks_desc[i] != three_of_a_kind_rank.unwrap() {
            return Some(vec![three_of_a_kind_rank.unwrap(), ranks_desc[i]]);
        }
    }

    return None;
}
