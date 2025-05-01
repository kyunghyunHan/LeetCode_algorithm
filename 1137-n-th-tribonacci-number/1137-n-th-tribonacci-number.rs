impl Solution {
   pub fn tribonacci(n: i32) -> i32 {
    let mut dp = vec![0; 38];

    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 1;
    for i in 3..=n {
        dp[i as usize] = dp[i as usize - 3] + dp[i as usize - 2] + dp[i as usize - 1];
    }
    dp[n as usize]
}
}