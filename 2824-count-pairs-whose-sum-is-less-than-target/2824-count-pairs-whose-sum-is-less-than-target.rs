impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut cnt = 0;

        for i in 0..nums.len(){
            for j in i..nums.len(){
               if nums[i] + nums[j] < target && i !=j{
                cnt+=1;
               }
            }
        }
        cnt
    }
}