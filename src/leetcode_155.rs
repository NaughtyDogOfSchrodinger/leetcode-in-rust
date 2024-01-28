struct MinStack {
    container: Vec<i32>,
    min: i32,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            container: vec![],
            min: i32::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        self.container.push(val);
        self.min = self.min.min(val);
    }

    fn pop(&mut self) {
        self.min = *self.container.iter().min().unwrap_or(&i32::MAX)
    }

    fn top(&self) -> i32 {
        *self.container.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_155::MinStack;

    #[test]
    fn test() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        println!("min: {}", min_stack.get_min());
        min_stack.pop();
        println!("min: {}", min_stack.top());
        println!("min: {}", min_stack.get_min());
    }
}
