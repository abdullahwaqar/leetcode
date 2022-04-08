struct MinStack {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push((val, val));
        } else {
            self.stack.push((val, val.min(self.get_min())));
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        return self.stack.last().unwrap().0;
    }

    fn get_min(&self) -> i32 {
        return self.stack.last().unwrap().1;
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 */
fn main() {
    let obj = MinStack::new();
    obj.push(100);
    obj.pop();
    let ret_3: i32 = obj.top();
    let ret_4: i32 = obj.get_min();
    println!("{:?}", ret_3);
}
