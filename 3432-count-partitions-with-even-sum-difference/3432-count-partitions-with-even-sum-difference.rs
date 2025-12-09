impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut n  = nums.len() -1;
        let mut sum_left = 0;
        let mut sum_right = 0;
        let mut difference = 0;
        let mut result = 0;

        for num in &nums{
            sum_right +=num;
        }

        for i in 0..n{
            sum_left += nums[i];
            sum_right -= nums[i];
            difference = sum_left - sum_right ;

            if difference % 2 == 0 {
                result+=1;
            }
        }
        result
    }
}