pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            _ => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn example_1() {
        let s = "()";
        assert_eq!(is_valid(s.into()), true);
    }

    #[test]
    fn example_2() {
        let s = "()[]{}";
        assert_eq!(is_valid(s.into()), true);
    }

    #[test]
    fn example_3() {
        let s = "(]";
        assert_eq!(is_valid(s.into()), false);
    }
}
