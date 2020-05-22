// question 59
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let n_square = n * n;
    let mut result = vec![vec![0; n]; n];

    let (mut row, mut col) = (0, 0);
    let mut current_direction = Direction::Right;

    let mut ceil = 0;
    let mut floor = n - 1;
    let mut left_bound = 0;
    let mut right_bound = n - 1;

    for i in 0..n_square {
        result[row][col] = i as i32;
        match current_direction {
            Direction::Right => {
                if col >= right_bound {
                    current_direction = Direction::Down;
                    row += 1;
                    ceil += 1;
                } else {
                    col += 1;
                }
            }
            Direction::Down => {
                if row >= floor {
                    current_direction = Direction::Left;
                    col -= 1;
                    right_bound -= 1;
                } else {
                    row += 1;
                }
            }
            Direction::Left => {
                if col <= left_bound {
                    current_direction = Direction::Up;
                    row -= 1;
                    floor -= 1;
                } else {
                    col -= 1;
                }
            }
            Direction::Up => {
                if row <= ceil {
                    current_direction = Direction::Right;
                    col += 1;
                    left_bound += 1;
                } else {
                    row -= 1;
                }
            }
        }
    }
    result
}
