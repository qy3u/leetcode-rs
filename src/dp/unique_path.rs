// question 62

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut grid = vec![vec![1; n]; m];
    for i in 1..m {
        for j in 1..n {
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
        }
    }

    grid[m - 1][n - 1]
}
