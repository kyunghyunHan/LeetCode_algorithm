use std::collections::HashMap;
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut count_map: HashMap<Vec<i32>, i32> = HashMap::new();

        for row in &matrix {
            let first = row[0];
            let pattern: Vec<i32> = row.iter().map(|&x| x ^ first).collect();
            *count_map.entry(pattern).or_insert(0) += 1;
        }

        *count_map.values().max().unwrap()
    }
}

