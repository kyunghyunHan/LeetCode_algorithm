impl Solution {
    pub fn transform_array( nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.iter().map(|x|{if x % 2 ==0 {
            0 }else{
                1
            }
        }).collect::<Vec<i32>>();
        nums.sort_by(|a,b|a.cmp(&b));
        nums

    }
}