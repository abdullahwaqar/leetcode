struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        return s
            .chars()
            .zip(s.chars().rev())
            .fold(true, |acc, (a, b)| acc & (a == b));
    }
}

fn main() {
    println!("Hello, world!");
}
