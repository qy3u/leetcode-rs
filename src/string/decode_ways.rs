// question 91
pub fn num_decodings(s: String) -> i32 {
    let mut result = 0;
    decode_helper(&s[..], &mut result);
    result
}

fn decode_helper(s: &str, result: &mut i32) {
    if s.len() == 0 {
        *result += 1;
    }

    for i in 1..=2 {
        if s.len() < i {
            return;
        }

        if let Ok(v) = s[0..i].parse::<u8>() {
            if i == 2 && v < 10 {
                return;
            }

            if v < 1 || v > 26 {
                return;
            }

            decode_helper(&s[i..], result);
        }
    }
}
