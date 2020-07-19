// question 240
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let mut row = 0;
    let mut col = matrix[0].len() - 1;

    while row < matrix.len() && col >= 0 {
        let val = matrix[row][col];
        if val == target {
            return true;
        } else if val < target {
            row += 1;
        } else {
            col -= 1;
        }
    }

    false
}
