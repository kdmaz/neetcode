use std::cmp::max;
use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = HashSet::from_iter(nums);
    let mut longest = 0;

    for num in &set {
        // has left value
        if set.contains(&(num - 1)) {
            continue;
        }

        let mut count = 1;
        // has right value
        while set.contains(&(num + count)) {
            count += 1;
        }

        longest = max(longest, count);
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::longest_consecutive;

    #[test]
    fn example_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive(nums), 9);
    }
}
