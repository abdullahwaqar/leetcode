struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            stack.push(match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                _ => {
                    if Some(c) == stack.pop() {
                        continue;
                    } else {
                        return false;
                    }
                }
            });
        }
        return stack.is_empty();
    }
}

fn main() {
    println!("{:?}", Solution::is_valid("()[]{}".to_string()));
    println!("{:?}", Solution::is_valid("(]".to_string()));
}
