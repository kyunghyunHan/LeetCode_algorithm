use std::collections::HashMap;
impl Solution {
   pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = HashMap::new();
        
        for &num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut total = 0;
        for (&num, &count) in freq.iter() {
            if count % k == 0 {
                total += num * count;
            }
        }

        total
    }
}