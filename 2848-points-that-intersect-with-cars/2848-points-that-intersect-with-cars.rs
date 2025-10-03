use std::collections::HashSet;

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
       let mut hs  = HashSet::new();
       for num in nums{
          for i in num[0]..=num[1]{
            hs.insert(i);
          }
       }
       hs.len() as i32
    }
}