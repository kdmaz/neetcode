use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map = HashMap::new();
    let (s, t) = (s.as_bytes(), t.as_bytes());

    for i in 0..s.len() {
        *map.entry(s[i]).or_insert(0) += 1;
        *map.entry(t[i]).or_insert(0) -= 1;
    }

    for value in map.values() {
        if *value != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn example_1() {
        let (s, t) = ("anagram".to_owned(), "nagaram".to_owned());
        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn example_2() {
        let (s, t) = ("rat".to_owned(), "car".to_owned());
        assert_eq!(is_anagram(s, t), false);
    }
}
