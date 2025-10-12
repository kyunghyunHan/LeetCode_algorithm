impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;

        for i in 0..(1<<n){
            let mut subset_xor = 0;
            for j in 0..n{
                if (i & (1<<j)) == 0{
                    subset_xor ^= nums[j];
                }
            }
            count +=subset_xor;

        }
        count
    }
}