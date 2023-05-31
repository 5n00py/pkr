use crate::card::Rank;

/// Finds a single pair and the kickers in descending order from the provided
/// ranks in descending order.
///
/// # Arguments
///
/// * `ranks_desc` - A vector of `Rank` values sorted in descending order.
///
/// # Returns
///
/// * `Some(Vec<Rank>)` - The pair and the kickers in descending order if found,
///   or `None` if not found.
pub fn find_pair(ranks_desc: &Vec<Rank>) -> Option<Vec<Rank>> {
    let ranks_len = ranks_desc.len();

    if ranks_len < 2 {
        return None;
    }

    let mut result = Vec::new();

    for i in 0..ranks_len - 1 {
        if ranks_desc[i] == ranks_desc[i + 1] {
            result.push(ranks_desc[i]);
            break;
        }
    }

    if result.len() == 1 {
        if ranks_len < 5 {
            let kickers: Vec<Rank> = ranks_desc
                .iter()
                .filter(|&&rank| rank != result[0])
                .copied()
                .collect();
            result.extend(kickers);
        } else {
            let kickers: Vec<Rank> = ranks_desc
                .iter()
                .filter(|&&rank| rank != result[0])
                .take(3) // Take the highest three kickers
                .copied()
                .collect();
            result.extend(kickers);
        }
        Some(result)
    } else {
        None
    }
}
