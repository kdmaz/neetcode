fn is_ascii_alphanumeric(byte: u8) -> bool {
    let upper = byte >= b'A' && byte <= b'Z';
    let lower = byte >= b'a' && byte <= b'z';
    let num = byte >= b'0' && byte <= b'9';
    upper || lower || num
}

pub fn is_palindrome(s: String) -> bool {
    let (mut l, mut r) = (0, s.len() - 1);
    let s = s.as_bytes();

    while l < r {
        if !is_ascii_alphanumeric(s[l]) {
            l += 1;
            continue;
        }

        if !is_ascii_alphanumeric(s[r]) {
            r -= 1;
            continue;
        }

        if (s[l] as char).to_ascii_lowercase() != (s[r] as char).to_ascii_lowercase() {
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
