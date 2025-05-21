use std::collections::{BinaryHeap, HashMap};

impl Solution {
fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut scores_map: HashMap<i32, Vec<i32>> = HashMap::new();

    // Group scores by student ID
    for item in items {
        let id = item[0];
        let score = item[1];
        scores_map.entry(id).or_insert(vec![]).push(score);
    }

    let mut result: Vec<Vec<i32>> = scores_map
        .into_iter()
        .map(|(id, mut scores)| {
            // Sort scores in descending order and take the top five
            scores.sort_unstable_by(|a, b| b.cmp(a));
            let top_five_sum: i32 = scores.into_iter().take(5).sum();
            vec![id, top_five_sum / 5]
        })
        .collect();

    // Sort by student ID
    result.sort_by_key(|entry| entry[0]);

    result
}
}