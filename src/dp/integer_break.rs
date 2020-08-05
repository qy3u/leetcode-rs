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

// 贪心
// pub fn integer_break(n: i32) -> i32 {
//     if n == 2 {
//         return 1;
//     }

//     if n == 3 {
//         return 2;
//     }

//     let i = n / 3;
//     let m = n % 3;

//     if m == 0 {
//         3i32.pow(i as u32)
//     } else if m == 1 {
//         3i32.pow((i - 1) as u32) * 4
//     } else {
//         3i32.pow(i as u32) * m
//     }
// }
