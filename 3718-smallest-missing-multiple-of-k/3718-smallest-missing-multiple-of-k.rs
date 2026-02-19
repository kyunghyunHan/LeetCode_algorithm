impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut res= 0;
        for i in 1..=200{
           let candidate = k * i;
            if !nums.contains(&candidate) {
                return candidate;
            }
        }
        0
      

    }
}