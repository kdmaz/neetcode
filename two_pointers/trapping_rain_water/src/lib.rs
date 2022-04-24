pub fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let (mut l, mut r) = (0, height.len() - 1);
    let (mut max_l, mut max_r) = (height[l], height[r]);
    let mut res = 0;

    while l < r {
        if max_l < max_r {
            l += 1;
            max_l = std::cmp::max(max_l, height[l]);
            res += max_l - height[l];
        } else {
            r -= 1;
            max_r = std::cmp::max(max_r, height[r]);
            res += max_r - height[r];
        }
    }

    res
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
