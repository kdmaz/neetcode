pub struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: vec![],
            min: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.stack.is_empty() || val <= self.get_min() {
            self.min.push(val);
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) {
        if self.stack.pop() == Some(self.get_min()) {
            self.min.pop();
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn example_1() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }
}
