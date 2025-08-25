use std::collections::HashSet;
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let sum_a: i32 = alice_sizes.iter().sum();
    let sum_b: i32 = bob_sizes.iter().sum();
    let delta = (sum_a - sum_b) / 2;

    let bob_set: HashSet<i32> = bob_sizes.into_iter().collect();

    for &x in &alice_sizes {
        let y = x - delta;
        if bob_set.contains(&y) {
            return vec![x, y];
        }
    }
    vec![]
    }
}
