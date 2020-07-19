// question 74 solution
// 1. My Solution

// pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
//     let m = matrix.len();
//     if m == 0 {
//         return false;
//     }

//     let n = matrix[0].len();
//     if n == 0 {
//         return false;
//     }

//     let row = matrix.binary_search_by(|row| row[0].cmp(&target));

//     if row.is_ok() {
//         return true;
//     }

//     let row = row.unwrap_or_else(|x| x);

//     if row == 0 {
//         return false;
//     }

//     return matrix[row - 1].binary_search(&target).is_ok();
// }
//

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    if m == 0 {
        return false;
    }

    let n = matrix[0].len();
    if n == 0 {
        return false;
    }

    let mut left = 0;
    let mut right = m * n;

    while left < right {
        let mid = left + (right - left) / 2;
        let coor = idx_to_coor(mid, n);
        let val = matrix[coor.0][coor.1];
        if val == target {
            return true;
        } else if val < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    false
}

fn idx_to_coor(idx: usize, col_num: usize) -> (usize, usize) {
    (idx / col_num, idx % col_num)
}
