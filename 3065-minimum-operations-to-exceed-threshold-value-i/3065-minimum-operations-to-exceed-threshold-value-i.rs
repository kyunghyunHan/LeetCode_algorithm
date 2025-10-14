impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        nums.sort_by(|a,b|a.cmp(&b));
        for num in nums{
            if num < k {
                count+=1;
            }
        }
        count
    }
}