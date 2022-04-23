pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;

    loop {
        let sum = numbers[l] + numbers[r];

        if sum > target {
            r -= 1;
        } else if sum < target {
            l += 1;
        } else {
            break;
        }
    }

    vec![l as i32 + 1, r as i32 + 1]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn example_1() {
        let numbers = vec![2, 7, 11, 15];
        assert_eq!(two_sum(numbers, 9), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let numbers = vec![2, 3, 4];
        assert_eq!(two_sum(numbers, 6), vec![1, 3]);
    }

    #[test]
    fn example_3() {
        let numbers = vec![-1, 0];
        assert_eq!(two_sum(numbers, -1), vec![1, 2]);
    }
}
