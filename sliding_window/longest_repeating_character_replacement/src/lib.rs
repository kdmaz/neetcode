pub fn character_replacement(s: String, k: i32) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::character_replacement;

    #[test]
    fn example_1() {
        let s = "ABAB";
        let k = 2;
        assert_eq!(character_replacement(s.into(), k), 4);
    }

    #[test]
    fn example_2() {
        let s = "AABABBA";
        let k = 1;
        assert_eq!(character_replacement(s.into(), k), 4);
    }
}
