impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.iter()
            .map(|x| x * x) // 정수 곱셈으로 제곱
            .collect::<Vec<i32>>();

        nums.sort(); // a.cmp(&b) 대신 이렇게 해도 됨
        nums
    }
}