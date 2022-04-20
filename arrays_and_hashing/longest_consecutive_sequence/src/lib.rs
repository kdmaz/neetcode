pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    todo!();
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
