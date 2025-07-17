impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> i32 {
        let majority_count = nums.len() / 2;

        for &num in &nums {
            let count = nums.iter().filter(|&&x| x == num).count();
            if count > majority_count {
                return num;
            }
        }

        // 문제 조건상 반드시 존재하므로 unreachable
        unreachable!()
    }
}