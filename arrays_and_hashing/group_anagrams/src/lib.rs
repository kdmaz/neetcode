use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for str in strs {
        let mut count = [0; 26];
        for c in str.chars() {
            count[(c as usize - 97)] += 1;
        }

        let bucket = map.entry(count).or_insert(vec![]);
        bucket.push(str.clone());
    }

    map.into_values().collect::<Vec<Vec<String>>>()
}

#[cfg(test)]
mod tests {
    use super::group_anagrams;

    #[test]
    fn example_1() {
        let strs = vec![
            "eat".to_owned(),
            "tea".to_owned(),
            "tan".to_owned(),
            "ate".to_owned(),
            "nat".to_owned(),
            "bat".to_owned(),
        ];

        let results = group_anagrams(strs);
        assert_eq!(results.len(), 3);
        for result in results {
            let one = result.contains(&"bat".to_owned());
            let two = result.contains(&"eat".to_owned())
                && result.contains(&"tea".to_owned())
                && result.contains(&"ate".to_owned());
            let three = result.contains(&"tan".to_owned()) && result.contains(&"nat".to_owned());
            assert!(one || two || three);
        }
    }

    #[test]
    fn example_2() {
        let strs = vec!["".to_owned()];
        assert_eq!(group_anagrams(strs), vec![vec!["".to_owned()]]);
    }

    #[test]
    fn example_3() {
        let strs = vec!["a".to_owned()];
        assert_eq!(group_anagrams(strs), vec![vec!["a".to_owned()]]);
    }
}
