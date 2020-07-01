// question 74 solution
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    if m == 0 {
        return false;
    }

    let n = matrix[0].len();
    if n == 0 {
        return false;
    }

    let row = matrix.binary_search_by(|row| row[0].cmp(&target));

    if row.is_ok() {
        return true;
    }

    let row = row.unwrap_or_else(|x| x);

    if row == 0 {
        return false;
    }

    return matrix[row - 1].binary_search(&target).is_ok();
}
