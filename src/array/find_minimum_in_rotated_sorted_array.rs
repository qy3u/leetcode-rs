
// question 153
pub fn find_min(nums: Vec<i32>) -> i32 {
    assert!(nums.len() > 0);

    let mut left = 0;
    let mut right = nums.len() - 1;

    let last = nums[right];


    while nums[left] > last {
        let mid = left + (right - left) / 2;
        if nums[mid] > last {
            left = mid + 1;
        }else{
            right = mid;
        }
    }

    nums[left]
}