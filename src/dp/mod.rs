mod best_time_to_buy_and_sell_stock_ii;
mod climbing_stairs;
mod coin_change;
mod counting_bits;
mod house_robber;
mod house_robber_ii;
mod integer_break;
mod jump_game;
mod longest_continuous_increasing_subsequence;
mod maximum_product_subarray;
mod min_path_sum;
mod regular_expression_matching;
mod unique_path;
mod unique_path_ii;

pub use best_time_to_buy_and_sell_stock_ii::max_profit;
pub use climbing_stairs::climb_stairs;
pub use coin_change::coin_change;
pub use counting_bits::count_bits;
pub use house_robber::rob;
pub use house_robber_ii::raw_rob;
pub use integer_break::integer_break;
pub use jump_game::can_jump;
pub use longest_continuous_increasing_subsequence::find_length_of_lcis;
pub use maximum_product_subarray::max_product;
pub use min_path_sum::min_path_sum;
pub use regular_expression_matching::is_match;
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
