// question 122
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut dp = vec![0; prices.len()];
    dp[0] = 0; // it's also 0 by default

    for i in 1..prices.len() {
        dp[i] = (prices[i] - prices[i - 1]).max(0) + dp[i - 1];
    }

    dp[prices.len() - 1]
}
