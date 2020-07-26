// question 89
pub fn gray_code(n: i32) -> Vec<i32> {
    if n == 0 {
        return vec![0];
    } else if n == 1 {
        return vec![0, 1];
    } else {
        let mut lower = gray_code(n - 1);
        let higher: Vec<i32> = lower
            .iter()
            .rev()
            .map(|x| x + 2_i32.pow((n - 1) as u32))
            .collect();
        lower.extend(higher);
        return lower;
    }
}
