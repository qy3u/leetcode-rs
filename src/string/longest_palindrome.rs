// question 409
pub fn longest_palindrome(s: String) -> i32 {
    let mut set = std::collections::HashSet::new();
    let mut ans = 0;

    for c in s.chars() {
        if set.contains(&c) {
            set.remove(&c);
            ans += 2;
        } else {
            set.insert(c);
        }
    }

    if !set.is_empty() {
        ans += 1;
    }
    ans
}
