impl Solution {
    pub fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
          let n = nums.len();
        let k = k as usize;

        let mut start = 0;

        for i in 1..=n - k {
            if &nums[i..i+k] > &nums[start..start+k] {
                start = i;
            }
        }

        nums[start..start+k].to_vec()
    }
    
}