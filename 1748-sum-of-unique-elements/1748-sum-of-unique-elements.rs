use std::collections::HashMap;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut freq = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut sum = 0;
    for (num, count) in freq {
        if count == 1 {
            sum += num;
        }
    }

    sum
}
}