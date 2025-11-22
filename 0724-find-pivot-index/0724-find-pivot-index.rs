impl Solution {
      pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut left = 0;

        for (i, &v) in nums.iter().enumerate() {
            let right = total - left - v;
            if left == right {
                return i as i32;
            }
            left += v;
        }

        -1
    }
}