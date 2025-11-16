use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut cnt = HashMap::new();
        for num in nums{
            *cnt.entry(num).or_insert(0)+=1;
        }

        for c in cnt.values(){
            if c % 2 !=0 {
               return false
            }
        }
        true
    }
}