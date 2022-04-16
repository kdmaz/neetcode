pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums), 9);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        assert_eq!(two_sum(nums), 6);
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        assert_eq!(two_sum(nums), 6);
    }
}
