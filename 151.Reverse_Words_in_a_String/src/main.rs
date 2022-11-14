struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        return s.split_whitespace().rev().collect::<Vec<_>>().join(" ");
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::reverse_words("the sky is blue".to_owned())
    );
}
