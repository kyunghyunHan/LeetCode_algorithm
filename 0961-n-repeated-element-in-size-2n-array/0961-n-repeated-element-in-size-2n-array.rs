impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
            let mut result: i32 = 0;
    for i in &nums {
        let count = &nums.iter().filter(|x| *x == i).count();
        if *count == (nums.len() / 2) {
            result = *i;
        }
    }
    result
    }
}