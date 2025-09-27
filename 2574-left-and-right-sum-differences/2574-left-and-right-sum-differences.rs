impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left_dp = vec![0;nums.len()+1];
        let mut right_dp = vec![0;nums.len()];
        let mut left_sum = 0;
        let mut right_sum = 0;
        let mut result =vec![];
        for i in 0..nums.len(){
             left_sum += nums[i];
             left_dp[i+1] += left_sum;
        }
        for i in (1..nums.len()).rev(){
             right_sum += nums[i];
             right_dp[i-1] = right_sum;
        }
        println!("{:?}",left_dp);

        println!("{:?}",right_dp);
        for i in 0..nums.len(){
            result.push((left_dp[i]-right_dp[i]).abs())
        }
        result
    }
}