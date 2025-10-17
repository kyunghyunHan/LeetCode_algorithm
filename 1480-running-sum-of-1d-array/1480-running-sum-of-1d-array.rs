impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut result = vec![];
        for num in nums{
            sum+=num;
            result.push(sum);
        }
        result
    }
}