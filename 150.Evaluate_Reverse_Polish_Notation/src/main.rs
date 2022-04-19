struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                stack.push(num);
                continue;
            }
            
            let num2 = stack.pop().unwrap();
            let num1 = stack.pop().unwrap();
            let res;
            if token == "+" {
                res = num1 + num2;
            } else if token == "-" {
                res = num1 - num2;
            } else if token == "*" {
                res = num1 * num2;
            } else if token == "/" {
                res = num1 / num2;
            } else {
                unreachable!();
            }
            stack.push(res);
        }
        return stack.pop().unwrap();
    }
}

fn main() {
    println!("Hello, world!");
}
