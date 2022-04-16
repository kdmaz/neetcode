use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();

    for c in s.chars() {
        if s_map.get(&c).is_some() {
            s_map.insert(c, s_map.get(&c).unwrap() + 1);
        } else {
            s_map.insert(c, 1);
        }
    }

    for c in t.chars() {
        if t_map.get(&c).is_some() {
            t_map.insert(c, t_map.get(&c).unwrap() + 1);
        } else {
            t_map.insert(c, 1);
        }
    }

    for c in s.chars() {
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
