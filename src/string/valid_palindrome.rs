// question 125
pub fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_digit(10))
        .collect();

    if chars.is_empty() {
        return true;
    }

    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() - 1 - i] {
            return false;
        }
    }

    true
}
