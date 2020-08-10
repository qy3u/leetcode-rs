// question 3
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut result = 0;

    let s = s.chars().collect::<Vec<char>>();
    let mut set = std::collections::HashSet::new();

    let mut j = 0;
    for i in 0..s.len() {
        while j < s.len() && !set.contains(&s[j]) {
            set.insert(s[j]);
            result = result.max((j - i + 1) as i32);
            j += 1;
        }

        set.remove(&s[i]);
    }

    result
}
