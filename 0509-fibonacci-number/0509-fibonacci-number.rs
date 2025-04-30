impl Solution {
    pub fn fib(n: i32) -> i32 {
    let mut dp = vec![0; 31];

    dp[2] = 1;
    dp[1] = 1;
    for i in 3..=30 {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n as usize]
}
}