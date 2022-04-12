struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = String::new();
        let mut sink = ' ';

        for c in command.chars() {
            match (c, sink) {
                ('G', _) => result.push(c),
                (')', '(') => result.push('o'),
                (')', 'l') => result.push_str("al"),
                _ => sink = c,
            }
        }

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::interpret("(al)G(al)()()G".to_owned()));
}
