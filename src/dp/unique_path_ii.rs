// question 63

// 1. 确定状态
//  1) 最后一步:
//      将到达上一个地方的 path 数相加
//  2) 子问题
//      如何到达上一个地方(遇到障碍怎么办)
//
// 2. 转移方程
//   dp[m][n] = dp[m-1][n] + dp[m][n-1]
//   if grid[m][n] == 1 => dp[m][n] = 0 (没有道路到达障碍)
//
// 3. 边界条件
//   dp[0][0]  = 1
//   if m==0 || n == 0
//      if grid[m][n] = 1
//          dp[m][n] = 0
//      else
//          dp[m][n] = 1
// 4. 计算顺序
//   从上到下, 从左到右
//
//
// (5). 错误总结:
//  边界情况考虑的不够清楚:
//      只有一行怎么处理
//      只有一列怎么处理
//

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    if m == 0 {
        return 0;
    }

    let n = obstacle_grid[0].len();
    if n == 0 {
        return 0;
    }

    if obstacle_grid[0][0] == 1 {
        return 0;
    }

    let mut dp = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            if obstacle_grid[i][j] == 1 {
                dp[i][j] = 0;
                continue;
            }

            dp[i][j] = if i == 0 && j == 0 {
                1
            } else if i == 0 {
                dp[i][j - 1]
            } else if j == 0 {
                dp[i - 1][j]
            } else {
                dp[i - 1][j] + dp[i][j - 1]
            };
        }
    }

    dp[m - 1][n - 1]
}
