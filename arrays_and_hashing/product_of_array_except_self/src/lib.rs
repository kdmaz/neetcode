pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix = vec![nums[0]];
    let mut postfix = vec![nums[nums.len() - 1]];
    let mut result = vec![];

    for i in 1..nums.len() {
        prefix.push(prefix[i - 1] * nums[i]);
        postfix.push(postfix[i - 1] * nums[nums.len() - 1 - i]);
    }
    postfix.reverse();

    for i in 0..nums.len() {
        let before = if i == 0 { 1 } else { prefix[i - 1] };
        let after = if i == nums.len() - 1 {
            1
        } else {
            postfix[i + 1]
        };

        result.push(before * after);
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
