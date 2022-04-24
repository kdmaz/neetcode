pub fn trap(height: Vec<i32>) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::trap;

    #[test]
    fn example_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(height), 6);
    }

    #[test]
    fn example_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(trap(height), 9);
    }
}
