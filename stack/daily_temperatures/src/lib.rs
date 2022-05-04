struct Pair {
    i: usize,
    temp: i32,
}

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<Pair> = vec![];
    let mut res = vec![0; temperatures.len()];

    for (i, temp) in temperatures.iter().enumerate() {
        let temp = *temp;
        while !stack.is_empty() && temp > stack.last().unwrap().temp {
            let stack_i = stack.pop().unwrap().i;
            res[stack_i] = (i - stack_i) as i32;
        }

        stack.push(Pair { i, temp });
    }

    res
}

#[cfg(test)]
mod tests {
    use super::daily_temperatures;

    #[test]
    fn example_1() {
        assert_eq!(
            daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73].into()),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            daily_temperatures([30, 40, 50, 60].into()),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(daily_temperatures([30, 60, 90].into()), vec![1, 1, 0]);
    }
}
