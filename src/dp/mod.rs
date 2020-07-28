mod coin_change;
mod jump_game;
mod longest_continuous_increasing_subsequence;
mod maximum_product_subarray;
mod min_path_sum;
mod unique_path;
mod unique_path_ii;

pub use coin_change::coin_change;
pub use jump_game::can_jump;
pub use longest_continuous_increasing_subsequence::find_length_of_lcis;
pub use maximum_product_subarray::max_product;
pub use min_path_sum::min_path_sum;
pub use unique_path::unique_paths;
pub use unique_path_ii::unique_paths_with_obstacles;

// 1. 确定状态
//  1) 最后一步
//  2) 子问题
//
//
// 2. 转移方程
//
// 3. 初始条件和边界情况
//
// 4. 计算顺序
