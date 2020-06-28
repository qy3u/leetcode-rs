// question 152
pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut dp_min = vec![0; nums.len()];
    let mut dp_max = vec![0; nums.len()];

    dp_min[0] = nums[0];
    dp_max[0] = nums[0];
    let mut ans = nums[0];

    for i in 1..nums.len() {
        dp_min[i] = vec![dp_min[i - 1] * nums[i], dp_max[i - 1] * nums[i], nums[i]]
            .into_iter()
            .min()
            .unwrap();
        dp_max[i] = vec![dp_min[i - 1] * nums[i], dp_max[i - 1] * nums[i], nums[i]]
            .into_iter()
            .max()
            .unwrap();

        ans = i32::max(dp_max[i], ans);
    }
    ans
}
