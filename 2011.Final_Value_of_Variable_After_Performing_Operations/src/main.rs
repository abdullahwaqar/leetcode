struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut val = 0;

        operations.iter().for_each(|operation| {
            if operation.contains("+") {
                val += 1;
            } else {
                val -= 1;
            }
        });

        return val;
    }
}

fn main() {
    println!("Hello, world!");
}
