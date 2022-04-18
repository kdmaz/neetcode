pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());

    let mut prefix = 1;
    for i in 0..nums.len() {
        result.push(prefix);
        prefix *= nums[i];
    }

    let mut postfix = 1;
    for i in (0..nums.len()).rev() {
        result[i] *= postfix;
        postfix *= nums[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::product_except_self;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(nums), vec![24, 12, 8, 6]);
    }

    #[test]
    fn example_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(product_except_self(nums), vec![0, 0, 9, 0, 0]);
    }
}
