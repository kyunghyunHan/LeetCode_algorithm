impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
          nums.sort_by(|a, b| {
    
        let a_is_even = a % 2 == 0;
        let b_is_even = b % 2 == 0;

        b_is_even.cmp(&a_is_even) 
    });
    nums
    }
}