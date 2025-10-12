impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        let mut pointer = nums.len()-1;
        nums.sort_by(|a,b|a.cmp(&b));

        
        let mut pointer: i32 = (nums.len() - 1) as i32;

        while pointer >= 0 {
            let left_sum: i32 = nums[..pointer as usize].iter().sum();
            let right_sum: i32 = nums[pointer as usize..].iter().sum();

            if right_sum > left_sum {
                let mut result: Vec<i32> = nums[pointer as usize..].to_vec();
                result.sort_by(|a, b| b.cmp(a)); // 내림차순 정렬
                return result;
            } else {
                pointer -= 1;
            }
        }
        Vec::new()
    }
}