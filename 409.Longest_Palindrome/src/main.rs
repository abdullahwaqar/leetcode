struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        s.bytes()
            .fold(std::collections::HashMap::new(), |mut cnts, x| {
                *cnts.entry(x).or_insert(0) += 1;
                cnts
            })
            .iter()
            .fold(0, |acc, (_, x)| acc + x / 2 * 2)
            .min(s.len() as i32 - 1)
            + 1
    }
}

fn main() {
    println!("Hello, world!");
}
