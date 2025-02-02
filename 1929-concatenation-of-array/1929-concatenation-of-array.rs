impl Solution {
   pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut v = vec![0; &nums.len() * 2];
    for i in 0..nums.len() {
        v[i] = nums[i];
        v[i + len ] = nums[i];
    }
    v
}
}