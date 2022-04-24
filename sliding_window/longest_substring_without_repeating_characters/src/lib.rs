use std::cmp::max;
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0;
    let mut set = HashSet::new();
    let mut l = 0;

    for r in 0..s.len() {
        let c = &s[r..r + 1];

        while set.contains(c) {
            set.remove(&s[l..l + 1]);
            l += 1;
        }

        set.insert(c);
        longest = max(longest, set.len());
    }

    longest as i32
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
