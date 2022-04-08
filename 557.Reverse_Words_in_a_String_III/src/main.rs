struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        return s
            .split(' ')
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ");
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::reverse_words("Let's take LeetCode contest".to_string())
    );
}
