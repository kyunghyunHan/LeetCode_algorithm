use std::collections::HashMap;
impl Solution {
pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut map = HashMap::new();

    for item in items1.into_iter().chain(items2) {
        let key = item[0];
        let value = item[1];
        *map.entry(key).or_insert(0) += value;
    }

    let mut result: Vec<Vec<i32>> = map.into_iter().map(|(k, v)| vec![k, v]).collect();

    result.sort_by(|a, b| a[0].cmp(&b[0]));
    result
}

}