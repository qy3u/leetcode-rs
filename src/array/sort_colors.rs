// question 75
pub fn sort_colors(nums: &mut Vec<i32>) {
    if nums.len() == 0 || nums.len() == 1 {
        return;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    for i in 0..nums.len() {
        if i > right {
            break;
        }
        loop {
            if left > i || i > right {
                break;
            }

            if nums[i] == 0 {
                nums.swap(i, left);
                left += 1;
            } else if nums[i] == 2 && right >= 1 {
                nums.swap(i, right);
                right -= 1;
            } else {
                break;
            }
        }
    }
}
