impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
            let mut new_v = Vec::new();
    for i in 0..nums.len()/2{
       
       new_v.push(nums[i]);
       new_v.push(nums[n as usize+i]);
    }
    new_v
    }
}