impl Solution {
    pub fn or_array(nums: Vec<i32>) -> Vec<i32> {
          let mut result = Vec::new();
    for i in 1..nums.len() {
        result.push(nums[i - 1] | nums[i]);
    }
    result
    }
}