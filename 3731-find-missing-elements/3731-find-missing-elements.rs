impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|a,b|a.cmp(&b));
        
         let mut result = vec![];

        for i in 1..nums.len() {
            let diff = nums[i] - nums[i - 1];
            if diff > 1 {
                for x in nums[i - 1] + 1..nums[i] {
                    result.push(x);
                }
            }
        }

        result
    }
}