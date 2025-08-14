impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
     let mut result = Vec::new();
    for i in 0..nums.len(){
        let mut count = 0;

        for j in 0..nums.len(){
            if i ==j{
                continue;
            }
            
            if nums[i]>nums[j]{
                count+=1;
            }
        }
        result.push(count);
    }
    result
    }
}