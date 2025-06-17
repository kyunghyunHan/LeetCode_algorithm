impl Solution {
  pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_by(|a, b| a.cmp(b));
    let mut max_sum = 0;
    for i in (0..nums.len()).step_by(2) {
        max_sum += nums[i];
    }

    max_sum
}
}