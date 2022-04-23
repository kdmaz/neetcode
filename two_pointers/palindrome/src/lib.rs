pub fn is_palindrome(s: String) -> bool {
    todo!();
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
