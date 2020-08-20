

// question 47
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![];
    }
    let mut nums = nums;
    nums.sort();
    let mut result = vec![nums.clone()];

    while !next_permutation(&mut nums[..]) {
        result.push(nums.clone());
    }

    result
}

fn next_permutation(nums: &mut [i32]) -> bool {
    assert!(nums.len() > 0);
    if nums.len() == 1 {
        return true;
    }

    let mut un_order = nums.len();
    for i in (0..=nums.len() - 2).rev() {
        if nums[i] < nums[i + 1] {
            un_order = i;
            break;
        }
    }

    if un_order == nums.len() {
        return true;
    }

    for i in (un_order + 1..=nums.len() - 1).rev() {
        if nums[i] > nums[un_order] {
            nums.swap(i, un_order);
            break;
        }
    }

    nums[un_order + 1..].reverse();

    false
}
