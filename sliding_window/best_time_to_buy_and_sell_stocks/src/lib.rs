pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, 1);
    let mut profit = 0;

    while r < prices.len() {
        if prices[l] < prices[r] {
            profit = std::cmp::max(profit, prices[r] - prices[l]);
        } else {
            l = r;
        }

        r += 1;
    }

    profit
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
