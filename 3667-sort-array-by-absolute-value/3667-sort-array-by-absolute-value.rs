impl Solution {
    pub fn sort_by_absolute_value(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|a,b|a.abs().cmp(&b.abs()));
        nums
    }
}