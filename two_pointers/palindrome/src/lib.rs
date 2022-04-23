fn is_alpha(l: &str) -> bool {
    let b = l.as_bytes()[0];
    let is_num = b >= "0".as_bytes()[0] && b <= "9".as_bytes()[0];
    let is_lowercase_letter = b >= "a".as_bytes()[0] && b <= "z".as_bytes()[0];
    is_num || is_lowercase_letter
}

pub fn is_palindrome(s: String) -> bool {
    let mut l = 0;
    let mut r = s.len() - 1;

    while l < r {
        let left_char = &s[l..l + 1].to_lowercase();
        if !is_alpha(left_char) {
            l += 1;
            continue;
        }

        let right_char = &s[r..r + 1].to_lowercase();
        if !is_alpha(right_char) {
            r -= 1;
            continue;
        }

        if left_char != right_char {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn example_1() {
        let s = "A man, a plan, a canal: Panama".to_owned();
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn example_2() {
        let s = "raceacar".to_owned();
        assert_eq!(is_palindrome(s), false);
    }

    #[test]
    fn example_3() {
        let s = " ".to_owned();
        assert_eq!(is_palindrome(s), true);
    }
}
