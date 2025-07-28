 use std::collections::HashMap;  

impl Solution {
pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
    if suits.iter().all(|&s| s == suits[0]) {
        return "Flush".to_string();
    }

    let mut rank_counts = HashMap::new();
    for rank in ranks {
        *rank_counts.entry(rank).or_insert(0) += 1;
    }

    let max_count = rank_counts.values().cloned().max().unwrap_or(0);
    match max_count {
        3..=5 => "Three of a Kind".to_string(),
        2 => "Pair".to_string(),
        _ => "High Card".to_string(),
    }
}
}