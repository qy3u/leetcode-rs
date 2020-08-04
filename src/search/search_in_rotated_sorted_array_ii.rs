
// question 81
pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return true;
        }

        if nums[left] < nums[mid] {
            if nums[left] <= target && target < nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        } else if nums[mid] < nums[right] {
            if nums[mid] < target && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        } else {
            if nums[mid] == nums[left] {
                left += 1;
            }

            if nums[mid] == nums[right] {
                right -= 1;
            }
        }
    }

    nums[left] == target
}
