// question 674
pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max = 1;

    let mut dp = vec![1; nums.len()];
    dp[0] = 1;

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            dp[i] = dp[i - 1] + 1;
            max = dp[i].max(max);
        }
    }

    return max;
}
