use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut map = HashMap::new();
    let mut longest = 0;
    let mut l = 0;
    let get_max = |max, v: &i32| std::cmp::max(max, *v);

    for r in 0..s.len() {
        *map.entry(&s[r..r + 1]).or_insert(0) += 1;

        while ((r - l + 1) as i32 - map.values().fold(0, get_max)) > k {
            *map.get_mut(&s[l..l + 1]).unwrap() -= 1;
            l += 1;
        }

        longest = std::cmp::max(r - l + 1, longest);
    }

    longest as i32
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
