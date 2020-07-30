// question 70
pub fn climb_stairs(n: i32) -> i32 {
    let n: usize = n as usize;
    if n == 0 {
        return 0;
    }
    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        if i == 1 || i == 2 {
            dp[i] = i as i32;
            continue;
        }

        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}
