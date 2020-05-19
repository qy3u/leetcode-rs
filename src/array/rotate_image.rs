// question 48
//
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut origin = std::collections::HashMap::<(usize, usize), i32>::new();
    if matrix.len() == 0 {
        return;
    }

    let row = matrix.len();
    let col = matrix[0].len();

    for x in 0..row {
        for y in 0..col {
            let origin_x_y = (x + 1, y + 1);
            let origin_val = origin.remove(&origin_x_y).unwrap_or(matrix[x][y]);
            let (target_x, target_y) = coor_after_rotate(row, origin_x_y);
            origin.insert((target_x, target_y), matrix[target_x - 1][target_y - 1]);
            matrix[target_x - 1][target_y - 1] = origin_val;
        }
    }
}

#[inline]
fn coor_after_rotate(row: usize, (x, y): (usize, usize)) -> (usize, usize) {
    // row 3 col 3
    // 1,1 -> 1,3
    // 1,2 -> 2,3
    // 1,3 -> 3,3
    // 2,1 -> 1,2
    // 2,2 -> 2,2
    // 2,3 -> 3,2
    // 3,1 -> 1,1
    // 3,2 -> 2,1
    // 3,3 -> 3,1
    //
    // (x, y) -> (y, x) -> (y, row - x + 1)
    // y -> x -> row - x + 1

    // 4, 4
    // 1, 1 -> 1, 4
    // 2, 2 -> 2, 3
    // 2, 3 -> 3, 2
    // 3,1 -> 1,2

    (y, row - x + 1)
}
