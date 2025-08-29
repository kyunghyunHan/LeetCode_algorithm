impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut operation = 0;
        for i in 0..nums.len()-1{
            if nums[i] >= nums[i+1]{
                let mut need = nums[i]+1 - nums[i+1];
                operation +=need;
                nums[i+1] = nums[i]+1;
            }
        }
        operation
    }
}