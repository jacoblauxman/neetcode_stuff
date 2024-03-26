// # 155 Min Stack
// note: design stack that supports push, pop, top, and retrieving minimum ele in constant time
#[allow(dead_code)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() && self.min_stack.is_empty() {
            self.stack.push(val);
            self.min_stack.push(val);
        } else {
            let min_val = val.min(*self.min_stack.last().unwrap());
            self.stack.push(val);
            self.min_stack.push(min_val);
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> &i32 {
        self.stack.last().unwrap()
    }

    fn get_min(&self) -> &i32 {
        self.min_stack.last().unwrap()
    }
}
