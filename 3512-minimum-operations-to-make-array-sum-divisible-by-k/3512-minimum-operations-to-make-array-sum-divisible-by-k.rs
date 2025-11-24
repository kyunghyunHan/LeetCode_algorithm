impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for num in nums{
            sum+=num;
        }
        sum % k
    }
}