// question 55

pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        true;
    }

    let end = nums.len() - 1;
    for j in (0..=end).rev() {
        // dont't check the last one
        if j == end {
            continue;
        }
        if nums[j] == 0 {
            if !can_jump_throuth_this_zero(&nums[..], j) {
                return false;
            }
        }
    }

    true
}

fn can_jump_throuth_this_zero(nums: &[i32], j: usize) -> bool {
    for i in (0..=j).rev() {
        let min_jump = j - i;
        if nums[i] as usize > min_jump {
            return true;
        }
    }
    false
}
