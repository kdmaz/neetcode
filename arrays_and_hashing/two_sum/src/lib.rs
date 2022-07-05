use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(num) = map.get(num) {
            return vec![*num, i as i32];
        }

        map.insert(target - *num, i as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        let expected = vec![0, 1];
        assert_eq!(two_sum(nums, 9), expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let expected = vec![1, 2];
        assert_eq!(two_sum(nums, 6), expected);
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let expected = vec![0, 1];
        assert_eq!(two_sum(nums, 6), expected);
    }
}
