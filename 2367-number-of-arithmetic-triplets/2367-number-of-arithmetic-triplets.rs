impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
           let mut cnt = 0;
           for i in 0..nums.len(){
            for j in i+1..nums.len(){
                for k in j+1..nums.len(){
                    if nums[j] - nums[i] ==diff && nums[k] - nums[j]==diff{
                    cnt+=1;
                    }
                }
            }
           }
           cnt
    }
}