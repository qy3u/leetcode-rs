// question 213
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    if nums.len() == 1 {
        return nums[0];
    }

    raw_rob(&nums[1..nums.len()]).max(raw_rob(&nums[0..nums.len() - 1]))
}

pub fn raw_rob(nums: &[i32]) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut dp = vec![vec![0i32; nums.len() + 1]; 2];

    for i in 1..=nums.len() {
        dp[0][i] = dp[0][i - 1].max(dp[1][i - 1]);
        dp[1][i] = dp[0][i - 1] + nums[i - 1];
    }

    dp[0][nums.len()].max(dp[1][nums.len()])
}
