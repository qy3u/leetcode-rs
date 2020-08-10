// question 209
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut result = std::i32::MAX;

    let mut j = 0;
    for i in 0..nums.len() {
        while j < nums.len() && sum < s {
            sum += nums[j];
            j += 1;
        }

        if sum >= s {
            result = result.min((j - i) as i32);
        }

        sum -= nums[i];
    }

    if result == std::i32::MAX {
        return 0;
    }

    result
}
