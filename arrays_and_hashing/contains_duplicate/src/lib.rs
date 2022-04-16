use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for i in 0..nums.len() {
        if set.contains(&nums[i]) {
            return true;
        }

        set.insert(nums[i]);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::contains_duplicate;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    fn example_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate(nums), true);
    }
}
