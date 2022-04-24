pub fn length_of_longest_substring(s: String) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn example_1() {
        let s = "abcabcbb";
        assert_eq!(length_of_longest_substring(s.into()), 3);
    }

    #[test]
    fn example_2() {
        let s = "bbbbb";
        assert_eq!(length_of_longest_substring(s.into()), 1);
    }
}
