// question 77
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let path = vec![];
    let remain: Vec<i32> = (1..=n).collect();
    combine_helper(k, &path, &remain[..], &mut result);
    result
}

fn combine_helper(k: i32, path: &Vec<i32>, remain: &[i32], result: &mut Vec<Vec<i32>>) {
    if k == 0 {
        result.push(path.clone());
    }

    if remain.len() == 0 {
        return;
    }

    for i in 0..remain.len() {
        let mut path = path.clone();
        path.push(remain[i]);
        combine_helper(k - 1, &path, &remain[i + 1..], result);
    }
}
