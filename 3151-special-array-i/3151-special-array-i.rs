impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
      let mut result = true;
      for i in 1..nums.len(){
        if nums[i] % 2 == 0 && nums[i-1] % 2 ==0{
            result = false;
        }
        if nums[i] % 2 != 0 && nums[i-1] % 2 !=0{
            result = false;

        }
      }
      result
    }
}