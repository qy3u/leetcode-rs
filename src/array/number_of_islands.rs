// question 200
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    let mut grid = grid;
    if grid.is_empty() {
        return result;
    }

    if grid[0].is_empty() {
        return result;
    }

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '1' {
                mark_grid(&mut grid, row, col);
                result += 1;
            }
        }
    }
    result
}

fn mark_grid(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
    if row >= grid.len() || col >= grid[0].len() {
        return;
    }

    if grid[row][col] == '0' || grid[row][col] == '*' {
        return;
    }

    grid[row][col] = '*';

    if row > 0 {
        mark_grid(grid, row - 1, col);
    }

    if row + 1 < grid.len() {
        mark_grid(grid, row + 1, col)
    }

    if col > 0 {
        mark_grid(grid, row, col - 1);
    }

    if col + 1 < grid[0].len() {
        mark_grid(grid, row, col + 1)
    }
}
