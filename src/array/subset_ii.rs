// question 90
pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut result = vec![];
    let path = vec![];
    subset_dfs(path, &mut result, &nums);
    result
}

fn subset_dfs(path: Vec<i32>, result: &mut Vec<Vec<i32>>, set: &[i32]) {
    result.push(path.clone());

    let mut prev = None;
    for i in 0..set.len() {
        let mut path = path.clone();
        if let Some(v) = prev {
            if v == set[i] {
                continue;
            }
        }

        path.push(set[i]);
        subset_dfs(path, result, &set[i + 1..]);
        prev = Some(set[i]);
    }
}

