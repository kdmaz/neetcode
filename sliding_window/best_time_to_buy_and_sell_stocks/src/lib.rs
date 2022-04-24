pub fn max_profit(prices: Vec<i32>) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_1() {
        let prices = [7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices.into()), 5);
    }

    #[test]
    fn example_2() {
        let prices = [7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices.into()), 0);
    }
}
