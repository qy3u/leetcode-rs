// question 73
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    if m == 0 {
        return;
    }

    let n = matrix[0].len();
    if n == 0 {
        return;
    }

    let mut first_row_to_zero = false;
    let mut first_col_to_zero = false;

    for i in 0..m {
        if matrix[i][0] == 0 {
            first_col_to_zero = true;
        }
    }

    for j in 0..n {
        if matrix[0][j] == 0 {
            first_row_to_zero = true;
        }
    }

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                // set first line in j to zero
                matrix[0][j] = 0;
                matrix[i][0] = 0;
            }
        }
    }

    for i in 1..m {
        if matrix[i][0] == 0 {
            for j in 0..n {
                matrix[i][j] = 0;
            }
        }
    }

    for j in 1..n {
        if matrix[0][j] == 0 {
            for i in 1..m {
                matrix[i][j] = 0;
            }
        }
    }

    if first_row_to_zero {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }

    if first_col_to_zero {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }
}
