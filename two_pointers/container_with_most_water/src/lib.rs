use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut highest_area = 0;

    let (mut l, mut r) = (0, height.len() - 1);

    while l < r {
        let area = min(height[l], height[r]) * (r - l) as i32;
        highest_area = max(highest_area, area);

        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    highest_area
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
