
// question 50
pub fn my_pow(x: f64, n: i32) -> f64 {
    let mut x = x;
    let mut n = n;

    if n < 0 && n != i32::MIN {
        x = 1.0 / x;
        n = -n;
    }

    if n == 0 {
        return 1.0;
    }

    if n % 2 == 0 {
        my_pow(x * x, n / 2)
    } else {
        x * my_pow(x, n - 1)
    }
}
