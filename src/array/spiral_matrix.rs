// question 54
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut current_direction = Direction::Right;
    if matrix.len() == 0 {
        return vec![];
    }
    let mut left_end = 0;
    let mut right_end = matrix[0].len() - 1;
    let mut bottom = matrix.len() - 1;
    let mut ceil = 0;

    let (mut row, mut col) = (0, 0);
    let mut ans = vec![];

    let total = matrix.len() * matrix[0].len();
    let (mut last_row, mut last_col) = (row, col);
    loop {
        ans.push(matrix[row][col]);
        if ans.len() == total {
            break;
        }

        // TODO: a nature break condition
        match current_direction {
            Direction::Right => {
                if col >= right_end {
                    row += 1;
                    ceil += 1;
                    current_direction = Direction::Down;
                } else {
                    col += 1;
                }
            }
            Direction::Down => {
                if row >= bottom {
                    col -= 1;
                    right_end -= 1;
                    current_direction = Direction::Left;
                } else {
                    row += 1;
                }
            }
            Direction::Left => {
                if col <= left_end {
                    row -= 1;
                    bottom -= 1;
                    current_direction = Direction::Up;
                } else {
                    col -= 1;
                }
            }
            Direction::Up => {
                if row <= ceil {
                    col += 1;
                    left_end += 1;
                    current_direction = Direction::Right;
                } else {
                    row -= 1;
                }
            }
        }

        if (last_row, last_col) == (row, col) {
            break;
        }

        last_row = row;
        last_col = col;
    }
    ans
}
