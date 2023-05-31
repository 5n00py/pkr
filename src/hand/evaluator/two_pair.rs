use crate::card::Rank;

/// Finds the two pairs and the kicker in descending order from the provided
/// ranks in descending order.
///
/// # Arguments
///
/// * `ranks_desc` - A vector of `Rank` values sorted in descending order.
///
/// # Returns
///
/// * `Some(Vec<Rank>)` - The two pairs and the kicker in descending order if
///   found, or `None` if not found.
pub fn find_two_pair(ranks_desc: &Vec<Rank>) -> Option<Vec<Rank>> {
    let ranks_len = ranks_desc.len();

    if ranks_len < 4 {
        return None;
    }

    let mut result = Vec::new();

    for i in 0..ranks_len - 1 {
        if ranks_desc[i] == ranks_desc[i + 1] {
            result.push(ranks_desc[i]);
            if result.len() == 2 {
                break;
            }
        }
    }

    if result.len() == 2 {
        if ranks_len > 4 {
            let kicker: Vec<Rank> = ranks_desc
                .iter()
                .filter(|&&rank| !result.contains(&rank))
                .take(1) // Take the highest kicker
                .copied()
                .collect();
            result.extend(kicker);
        }
        Some(result)
    } else {
        None
    }
}
