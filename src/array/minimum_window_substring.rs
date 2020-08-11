
// question 76
pub fn min_window(s: String, t: String) -> String {
    let mut result = String::new();

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut remain = t.len();

    let mut origin_map = std::collections::HashMap::new();
    let mut remain_map = std::collections::HashMap::new();

    for c in t {
        *origin_map.entry(c).or_insert(0i32) += 1;
        *remain_map.entry(c).or_insert(0i32) += 1;
    }

    let mut j = 0;
    for i in 0..s.len() {
        while j < s.len() && remain != 0 {
            if let Some(count) = remain_map.get_mut(&s[j]) {
                if *count > 0 {
                    remain -= 1;
                }
                *count -= 1;
            }
            j += 1;
        }

        if remain == 0 {
            if result.len() == 0 || j - i < result.len() {
                result = s[i..j].iter().collect();
            }
        }

        if let Some(count) = remain_map.get_mut(&s[i]) {
            let max_count = *origin_map.get(&s[i]).unwrap();
            if *count < 0 {
                *count += 1;
            } else if *count >= max_count {
                // do nothing
            } else {
                *count += 1;
                remain += 1;
            }
        }
    }

    result
}
