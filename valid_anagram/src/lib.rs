use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let (mut s_map, mut t_map) = (HashMap::new(), HashMap::new());
    let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());

    for i in 0..s.len() {
        *s_map.entry(s_bytes[i] as char).or_insert(0) += 1;
        *t_map.entry(t_bytes[i] as char).or_insert(0) += 1;
    }

    for i in 0..s.len() {
        let c = s_bytes[i] as char;
        if s_map.get(&c) != t_map.get(&c) {
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
