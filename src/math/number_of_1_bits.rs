// question 191
pub fn hammingWeight(n: u32) -> i32 {
    if n == 0 {
        return 0;
    }

    let lowest = n & 0x1;
    lowest as i32 + hammingWeight(n / 2)
}
