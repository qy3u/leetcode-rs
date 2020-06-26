pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];

    dp[0] = 0;
    for i in 0..=amount {
        for j in 0..coins.len() {
            if i - coins[j] >= 0 {
                dp[i as usize] = i32::min(dp[i as usize], dp[(i - coins[j]) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] == amount + 1 {
        -1
    } else {
        dp[amount as usize]
    }
}
