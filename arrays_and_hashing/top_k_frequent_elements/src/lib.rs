use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    // index is the frequency a num occurs, value are a vec of nums that occur at that frequency
    // add an extra index to offset 0, this is so the index matches the frequency a num occurs
    // so index 1 means the nums in this vec occur 1 time
    // without the offset, index 0 would be used to mean the nums in this vec occur 1 time
    let mut frequency = vec![vec![]; nums.len() + 1];

    for num in nums {
        *map.entry(num).or_insert(0) += 1;
    }

    for (k, v) in map {
        // without the offset above, would require subtracting one from the value here
        frequency[v].push(k);
    }

    let mut result = vec![];

    for i in (0..frequency.len()).rev() {
        for n in &frequency[i] {
            result.push(*n);
            if result.len() as i32 == k {
                return result;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::top_k_frequent;

    #[test]
    fn example_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(top_k_frequent(nums, 2), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        assert_eq!(top_k_frequent(nums, 1), vec![1]);
    }
}
