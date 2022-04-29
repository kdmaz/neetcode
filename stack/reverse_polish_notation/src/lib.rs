pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut nums = vec![];
    let perform_operation = |operator: String, a, b| match operator.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => panic!("invalid operator"),
    };

    for t in tokens {
        let parsed_num = t.parse::<i32>();

        if parsed_num.is_ok() {
            nums.push(parsed_num.unwrap());
            continue;
        }

        let b = nums.pop().unwrap();
        let a = nums.pop().unwrap();
        nums.push(perform_operation(t, a, b))
    }

    *nums.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::eval_rpn;

    #[test]
    fn example_1() {
        let tokens = [
            "2".to_owned(),
            "1".to_owned(),
            "+".to_owned(),
            "3".to_owned(),
            "*".to_owned(),
        ];
        assert_eq!(eval_rpn(tokens.into()), 9);
    }

    #[test]
    fn example_2() {
        let tokens = [
            "4".to_owned(),
            "13".to_owned(),
            "5".to_owned(),
            "/".to_owned(),
            "+".to_owned(),
        ];
        assert_eq!(eval_rpn(tokens.into()), 6);
    }

    #[test]
    fn example_3() {
        let tokens = [
            "10".to_owned(),
            "6".to_owned(),
            "9".to_owned(),
            "3".to_owned(),
            "+".to_owned(),
            "-11".to_owned(),
            "*".to_owned(),
            "/".to_owned(),
            "*".to_owned(),
            "17".to_owned(),
            "+".to_owned(),
            "5".to_owned(),
            "+".to_owned(),
        ];
        assert_eq!(eval_rpn(tokens.into()), 22);
    }
}
