pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut l = i + 1;
        let mut r = nums.len() - 1;

        while l < r {
            let sum = nums[i] + nums[l] + nums[r];

            if sum < 0 {
                l += 1;
            } else if sum > 0 {
                r -= 1;
            } else {
                res.push(vec![nums[i], nums[l], nums[r]]);
                l += 1;

                while nums[l] == nums[l - 1] && l < r {
                    l += 1;
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::three_sum;

    #[test]
    fn example_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(three_sum(nums), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn example_2() {
        let nums = vec![];
        assert_eq!(three_sum(nums), vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn example_3() {
        let nums = vec![0];
        assert_eq!(three_sum(nums), vec![] as Vec<Vec<i32>>);
    }
}
