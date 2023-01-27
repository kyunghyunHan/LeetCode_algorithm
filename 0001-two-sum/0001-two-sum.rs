use std::collections::HashMap;
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut array = HashMap::new();
        for idx in 0..nums.len() {
            let num = nums[idx];
            let need = target - num;
            // If peer number exist in map, return the index.
            if let Some(found) = array.get(&need) {
                return vec![*found as i32, idx as i32];
            }
            array.insert(num, idx);
        }
        vec![]
    }
}