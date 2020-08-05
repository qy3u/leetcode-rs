
// question 343
pub fn integer_break(n: i32) -> i32 {
    assert!(n >= 2);

    let n: usize = n as usize;
    let mut dp = vec![0i32; n + 1];
    dp[1] = 1;
    dp[2] = 1;

    for i in 3..=n {
        dp[i] = (1..i)
            .into_iter()
            .flat_map(|x| {
                vec![
                    (x as i32) * dp[i - x],
                    dp[x] * dp[i - x],
                    x as i32 * (i as i32 - x as i32),
                ]
            })
            .max()
            .unwrap();
    }

    dp[n]
}
