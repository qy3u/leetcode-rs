// question 64
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    if m == 0 {
        return 0;
    }

    let n = grid[0].len();
    if n == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            dp[i][j] += if i == 0 && j == 0 {
                grid[i][j]
            } else if i == 0 {
                dp[i][j - 1] + grid[i][j]
            } else if j == 0 {
                dp[i - 1][j] + grid[i][j]
            } else {
                dp[i][j - 1].min(dp[i - 1][j]) + grid[i][j]
            }
        }
    }
    dp[m - 1][n - 1]
}
