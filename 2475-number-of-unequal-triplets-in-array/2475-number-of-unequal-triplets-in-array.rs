use std::collections::HashMap;
impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
         for n in &nums {
           *hm.entry(*n).or_insert(0) += 1;
         }

         let mut res = 0;
         let mut left = 0; 
         let mut right = nums.len();

         for (n,cnt) in hm{
            right -= cnt;
            res += left * cnt * right;
            left += cnt;
         }
         res as i32
       
    }
}