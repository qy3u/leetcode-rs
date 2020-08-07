
// question  121
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut min = i32::MAX;

    for i in prices {
        min = min.min(i);
        max = max.max(i - min);
    }
    max
}
