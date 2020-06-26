// question 55
pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mut dp = vec![false; nums.len()];
    dp[0] = true;

    for i in 1..nums.len() {
        for j in (0..i).rev() {
            if dp[j] && nums[j] >= (i - j) as i32 {
                dp[i] = true;
            }
        }
    }

    dp[nums.len() - 1]
}
