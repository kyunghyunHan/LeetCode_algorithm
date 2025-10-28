impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a,b|b.cmp(&a));
        let result = (nums[0]-1)* ( nums[1]-1);
        result
    }
}