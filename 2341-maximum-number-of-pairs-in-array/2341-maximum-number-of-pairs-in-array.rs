use std::collections::HashMap;

impl Solution {
  pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut mp = HashMap::new();

        for &x in &nums {
            *mp.entry(x).or_insert(0) += 1;
        }

        let mut n = nums.len() as i32;
        let mut c = 0; 

        for (_, count) in mp {
            if count >= 2 {
                let pairs = count / 2;
                c += pairs;
                n -= pairs * 2;
            }
        }

        vec![c, n]
    }
}