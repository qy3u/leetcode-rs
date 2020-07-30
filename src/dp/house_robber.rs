

// question 198
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut dp = vec![vec![0i32; nums.len() + 1]; 2];
    let not_robbed = 0;
    let robbed = 1;

    for i in 1..=nums.len() {
        dp[not_robbed][i] = dp[robbed][i - 1].max(dp[not_robbed][i - 1]);
        dp[robbed][i] = dp[not_robbed][i - 1] + nums[i - 1];
    }

    dp[not_robbed][nums.len()].max(dp[robbed][nums.len()])
}
