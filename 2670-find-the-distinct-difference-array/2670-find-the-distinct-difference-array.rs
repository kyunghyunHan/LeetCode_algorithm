use std::collections::HashSet;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut v = vec![];
        let mut hs1 = HashSet::new();

        for i in 0..nums.len(){
            let mut hs2 = HashSet::new();

            for j in i + 1..nums.len(){
                hs2.insert(nums[j]);
            }
            println!("{}",i);
            hs1.insert(nums[i]);

            // println!("{} {}",hs1.len(),hs2.len());

            let  r = (hs1.len() - hs2.len()) as i32;
            v.push(r)
        }
        v
    }
}