impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut dp = vec![0i32;  n  + 1];
    for i in 1..=n {


        dp[i] = dp[i >> 1] as i32 + (i & 1) as i32
    }
    dp

    }
}