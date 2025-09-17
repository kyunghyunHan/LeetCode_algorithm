use std::collections::HashSet;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut vec = vec![];
        for i in 0..nums.len(){
            for j in i..nums.len(){
                let mut hs = HashSet::new();
                for k  in i..=j{
                   hs.insert(nums[k]);
                }
              vec.push(hs);
            }
        }
        let mut result = 0;
        for i in 0..vec.len(){
            let value = vec[i].len() as i32;
            result+=value.pow(2);
        }
        println!("{:?}",vec);
        result
    }
}