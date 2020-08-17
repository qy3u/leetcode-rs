// question 10
pub fn is_match(s: String, p: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();

    if p.len() == 0 && s.len() != 0  {
        return false;
    }

    return match_helper(&s[..], &p[..]);

}


fn match_helper(s: &[char], p: &[char]) -> bool{
    if s.len() == 0 && p.len() == 0{
        return true;
    }

    if s.len() == 0 {
        if p.len() >= 2 && p[1] == '*'{
            return match_helper(&s[..], &p[2..]);
        }
        return false;
    }

    if (s.len() == 0 || p.len() == 0) && s.len() != p.len(){
        return false;
    }

    if p.len() == 1 {
        if p[0] == '.' || p[0] == s[0]{
            return match_helper(&s[1..], &p[1..]);
        }

        return false;
    }

    if p[1] != '*'{
        if p[0] == '.' || p[0] == s[0]{
            return match_helper(&s[1..], &p[1..]);
        }
        return false;
    }

    // p[1] == '*'
    if s[0] != p[0] && p[0] != '.' {
        return match_helper(&s[..], &p[2..]);
    }

    return match_helper(&s[1..], &p[..]) || match_helper(&s[1..], &p[2..]) || match_helper(&s[..], &p[2..]);
}