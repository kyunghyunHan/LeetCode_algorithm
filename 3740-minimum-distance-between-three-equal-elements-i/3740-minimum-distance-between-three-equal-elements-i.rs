impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut result =-1;
        let n = nums.len();
        let mut result: i32 = i32::MAX;

        if nums.len() < 3 {
            return -1;
        }
      for i in 0..n-2 {
            for j in i+1..n-1 {
                for k in j+1..n {
                    if nums[i] == nums[j] && nums[j] == nums[k] {
                       let dist = (i as i32 - j as i32).abs()
                                 + (j as i32 - k as i32).abs()
                                 + (k as i32 - i as i32).abs();
                        result = result.min(dist);
                 }
              }
           }
        }
        if result == i32::MAX { -1 } else { result }

        
    }
}