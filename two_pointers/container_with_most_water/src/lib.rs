pub fn max_area(height: Vec<i32>) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::max_area;

    #[test]
    fn example_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn example_2() {
        let height = vec![1, 1];
        assert_eq!(max_area(height), 1);
    }
}
