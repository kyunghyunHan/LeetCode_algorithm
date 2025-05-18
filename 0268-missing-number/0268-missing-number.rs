impl Solution {
   pub fn missing_number(nums: Vec<i32>) -> i32 {
    // let mut result = false;
    for i in 0..nums.len() {
        if !nums.contains(&(i as i32)) {
            // result = true;
            return i as i32;
        }
    }
    return nums.len() as i32;
}
}