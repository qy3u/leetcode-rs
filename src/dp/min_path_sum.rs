// question 64
// pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
//     let m = grid.len();
//     if m == 0 {
//         return 0;
//     }
//
//     let n = grid[0].len();
//     if n == 0 {
//         return 0;
//     }
//
//     let mut dp = vec![vec![0; n]; m];
//
//     for i in 0..m {
//         for j in 0..n {
//             dp[i][j] += if i == 0 && j == 0 {
//                 grid[i][j]
//             } else if i == 0 {
//                 dp[i][j - 1] + grid[i][j]
//             } else if j == 0 {
//                 dp[i - 1][j] + grid[i][j]
//             } else {
//                 dp[i][j - 1].min(dp[i - 1][j]) + grid[i][j]
//             }
//         }
//     }
//     dp[m - 1][n - 1]
// }

// 滚动数组
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    if m ==0{
        return 0;
    }

    let n = grid[0].len();
    if n==0{
        return 0;
    }

    let mut dp = vec![vec![0; n]; 2];
    let mut now = 0;
    let mut old = 1 - now;

    for i in 0..m{
        for j in 0..n{
            if i == 0 && j==0{
                dp[now][j] = grid[i][j];
                continue;
            }

            let min_sum = if i == 0 {
                dp[now][j-1]
            }else if j==0{
                dp[old][j]
            }else{
                dp[old][j].min(dp[now][j-1])
            };

            dp[now][j] = grid[i][j] + min_sum;
        }
        now = 1 - now;
        old = 1 - old;
    }

    dp[now][n-1]
}
