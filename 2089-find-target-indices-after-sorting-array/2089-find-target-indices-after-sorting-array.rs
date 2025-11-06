impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_by(|a,b|a.cmp(&b));
        let mut result = vec![];
        for i in 0..nums.len(){
            if nums[i]== target{
           result.push(i as i32);
            }
        }
        result
    }
}