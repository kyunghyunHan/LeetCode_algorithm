impl Solution {
    pub fn maximize_expression_of_three(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a,b|b.cmp(&a));

        nums[0] + nums[1] - nums[nums.len()-1]
    }
}