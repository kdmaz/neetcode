pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut stack = vec![];
    let mut res = vec![];
    backtrack(n, 0, 0, &mut stack, &mut res);
    res
}

fn backtrack(n: i32, open_n: i32, close_n: i32, stack: &mut Vec<String>, res: &mut Vec<String>) {
    if open_n == close_n && open_n == n {
        res.push(stack.join(""));
        return;
    }

    if open_n < n {
        stack.push("(".to_owned());
        backtrack(n, open_n + 1, close_n, stack, res);
        stack.pop();
    }

    if close_n < open_n {
        stack.push(")".to_owned());
        backtrack(n, open_n, close_n + 1, stack, res);
        stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::generate_parenthesis;

    #[test]
    fn example_1() {
        assert_eq!(
            generate_parenthesis(3),
            vec![
                "((()))".to_owned(),
                "(()())".to_owned(),
                "(())()".to_owned(),
                "()(())".to_owned(),
                "()()()".to_owned()
            ]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(generate_parenthesis(1), vec!["()".to_owned()]);
    }
}
