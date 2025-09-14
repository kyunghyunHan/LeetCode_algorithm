impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut temp = vec![0;nums.len()];
        for i in 0..nums.len(){
            temp[i] =nums[nums[i] as usize] as i32;
        }
        temp
    }
}