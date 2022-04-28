pub fn check_inclusion(s1: String, s2: String) -> bool {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::check_inclusion;

    #[test]
    fn example_1() {
        let (s1, s2) = ("ab", "eidbaooo");
        assert_eq!(check_inclusion(s1.into(), s2.into()), true);
    }

    #[test]
    fn example_2() {
        let (s1, s2) = ("ab", "eidboaoo");
        assert_eq!(check_inclusion(s1.into(), s2.into()), false);
    }

    #[test]
    fn example_3() {
        let (s1, s2) = ("hello", "ooolleoooleh");
        assert_eq!(check_inclusion(s1.into(), s2.into()), false);
    }

    #[test]
    fn example_4() {
        let (s1, s2) = ("adc", "dcda");
        assert_eq!(check_inclusion(s1.into(), s2.into()), true);
    }
}
