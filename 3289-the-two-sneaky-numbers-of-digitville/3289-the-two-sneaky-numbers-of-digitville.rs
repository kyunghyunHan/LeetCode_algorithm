use std::collections::HashMap;
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut hm = HashMap::new();
        for n in nums{
            *hm.entry(n).or_insert(0) += 1;
        }
        let mut result = vec![];
        for i in hm{
            if i.1 > 1 {
               result.push(i.0);
            }
        }
        result

    }
}