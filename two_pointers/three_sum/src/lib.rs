pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::three_sum;

    #[test]
    fn example_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(three_sum(nums), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn example_2() {
        let nums = vec![];
        assert_eq!(three_sum(nums), vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn example_3() {
        let nums = vec![0];
        assert_eq!(three_sum(nums), vec![] as Vec<Vec<i32>>);
    }
}
