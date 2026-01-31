impl Solution {
    pub fn max_k_distinct(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        
       nums.sort_by(|a,b|b.cmp(&a));
      let mut result = vec![nums[0]];

       for i in 1..nums.len(){
          if nums[i-1] != nums[i] && result.len() < k as usize{
              result.push(nums[i]);
          }
       }
       result
    }
}