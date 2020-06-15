// question 40

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    let mut result = vec![];
    let path = vec![];
    combination_sum2_dfs(&candidates[..], target, path, &mut result);
    result
}

fn combination_sum2_dfs(
    candidates: &[i32],
    target: i32,
    path: Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if target == 0 {
        result.push(path.clone());
    }

    if target < 0 {
        return;
    }

    for i in 0..candidates.len() {
        if i > 0 && candidates[i] == candidates[i - 1] {
            continue;
        }

        let elem = candidates[i];
        let mut path = path.clone();
        path.push(elem);

        if i != candidates.len() {
            combination_sum2_dfs(&candidates[i + 1..], target - elem, path, result);
        }
    }
}
