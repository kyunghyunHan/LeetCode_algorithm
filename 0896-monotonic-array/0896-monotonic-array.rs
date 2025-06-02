impl Solution {
  pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut store = 0;

    for i in 0..nums.len() - 1 {
        let c = nums[i].cmp(&nums[i + 1]) as i32;
        if c != 0 {
            if c != store && store != 0 {
                return false;
            }
            store = c;
        }
    }
    true
}
}