
// qeustion 338
pub fn count_bits(num: i32) -> Vec<i32> {
    let mut result = vec![0; num as usize + 1];
    for i in 1..=num {
        result[i as usize] = (i & 0x1) + result[(i / 2) as usize];
    }
    result
}
