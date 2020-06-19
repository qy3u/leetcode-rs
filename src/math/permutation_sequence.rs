// question 60
pub fn get_permutation(n: i32, k: i32) -> String {
    let mut result = String::new();
    let mut vals: Vec<i32> = (1..n + 1).collect();

    let c = k;
    for _ in 1..=n {
        let s = get_kth(&mut vals, c);
        result.push_str(&s);
    }

    result
}

fn get_kth(vals: &mut Vec<i32>, k: i32) -> String {
    if vals.len() == 1 {
        return vals.remove(0).to_string();
    }

    if vals.len() == 2 {
        if k % 2 == 0 {
            return vals.remove(1).to_string();
        }
        return vals.remove(0).to_string();
    }

    let group_num = fac(vals.len() - 1);
    let index = (k - 1) as usize / group_num % vals.len();

    vals.remove(index).to_string()
}

fn fac(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    n * fac(n - 1)
}
