use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut result = vec![];

    for i in 0..nums.len() {
        let num = nums[i];

        if map.get(&num).is_some() {
            result.push(*map.get(&num).unwrap() as i32);
            result.push(i as i32);
            break;
        } else {
            let inverse = target - num;
            map.insert(inverse, i);
        }
    }

    result
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
