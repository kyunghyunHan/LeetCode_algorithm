impl Solution {
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        
   
   for i in 0..nums.len(){
      if target==nums[i as usize]{
        return i as i32;
      }
   }
   return -1;
}
}