impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
          
           for _ in 0..k {
        if let Some(min_val) = nums.iter().min() {
            let idx = nums.iter().position(|x| x == min_val).unwrap();
            nums[idx] *= multiplier;
        }
    }
    nums
        
    }
}