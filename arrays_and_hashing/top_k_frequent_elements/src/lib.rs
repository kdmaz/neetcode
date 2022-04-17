pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::top_k_frequent;

    #[test]
    fn example_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(top_k_frequent(nums, 2), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        assert_eq!(top_k_frequent(nums, 1), vec![1]);
    }
}
