// question 287
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let n = nums.len() - 1;
    let mut left = 0;
    let mut right = n;

    while left <= right {
        let mid = left + (right - left / 2);
        let mut left_count = 0;
        let mut mid_count = 0;
        for v in &nums {
            if *v == mid as i32 {
                mid_count += 1;
            } else if *v >= left as i32 && *v < mid as i32 {
                left_count += 1;
            }
        }
        if mid_count > 1 {
            return mid as i32;
        } else if left_count > 1 {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}
