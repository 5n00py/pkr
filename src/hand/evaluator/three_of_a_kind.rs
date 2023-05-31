use crate::card::Rank;

/// Finds the highest three of a kind and the kickers in descending order from
/// the provided ranks in descending order.
///
/// # Arguments
///
/// * `ranks_desc` - A vector of `Rank` values sorted in descending order.
///
/// # Returns
///
/// * `Some(Vec<Rank>)` - The highest three of a kind and the kickers in
///   descending order if found or `None` if not found.
pub fn find_three_of_a_kind(ranks_desc: &Vec<Rank>) -> Option<Vec<Rank>> {
    let ranks_len = ranks_desc.len();
    if ranks_len < 3 {
        return None;
    }

    for i in 0..ranks_len - 2 {
        if ranks_desc[i] == ranks_desc[i + 2] {
            let mut result = vec![ranks_desc[i]];
            if ranks_len == 3 {
                return Some(result);
            } else if ranks_len == 4 {
                result.push(ranks_desc[(i + 3) % ranks_len]);
                return Some(result);
            } else {
                let kickers: Vec<Rank> = ranks_desc
                    .iter()
                    .filter(|&&rank| rank != ranks_desc[i])
                    .take(2) // Take the highest two kickers
                    .copied()
                    .collect();
                result.extend(kickers);
                return Some(result);
            }
        }
    }

    None
}
