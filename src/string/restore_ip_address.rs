// question 93
pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut result = Vec::new();
    helper(&s, vec![], &mut result);
    result
}

fn helper(s: &str, path: Vec<u8>, result: &mut Vec<String>) {
    if path.len() > 4 {
        return;
    }

    if s.len() == 0 && path.len() == 4 {
        result.push(
            path.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("."),
        );

        return;
    }

    for i in 1..=3 {
        if s.len() < i {
            break;
        }

        if let Ok(v) = s[0..i].parse::<u8>() {
            if i > 1 && v < 10 {
                return;
            }
            let mut path = path.clone();
            path.push(v);
            helper(&s[i..], path, result);
        }
    }
}
