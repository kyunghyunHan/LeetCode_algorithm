impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let mut max =i32::MIN;
        nums.sort();

        let n = nums.len();

        let a = nums[n - 1] * nums[n - 2] * nums[n - 3];

        let b = nums[0] * nums[1] * nums[n - 1];

        a.max(b)

      
    }
}