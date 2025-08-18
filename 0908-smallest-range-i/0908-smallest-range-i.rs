impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let max = nums.iter().max().unwrap();
    let min = nums.iter().min().unwrap();
    let max = 0.max(*max - min - 2 * k);
    max
    }
} 
