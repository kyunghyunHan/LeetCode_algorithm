impl Solution {
  pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        if i % 2 == 0 {
            if nums[i] % 2 != 0 {
                for j in i..nums.len() {
                    if nums[j] % 2 == 0 {
                        nums.swap(i, j);
                    }
                }
            }
        } else if i % 2 != 0 {
            if nums[i] % 2 == 0 {
                for j in i..nums.len() {
                    if nums[j] % 2 != 0 {
                        nums.swap(i, j);
                    }
                }
            }
        }
    }
    nums
}

}