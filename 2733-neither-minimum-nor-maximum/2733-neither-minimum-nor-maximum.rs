impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1
        }

        let a= nums[0].min(nums[1]);
        let b= nums[0].max(nums[1]);
        let c= nums[2];

        if c < b && c > a {
           return c
        }
        if c < a {
          return  a
        }
        if c > b{
          return  b
        }
        -1

    }
}