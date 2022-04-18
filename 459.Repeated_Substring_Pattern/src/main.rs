struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        return (s.clone() + &s)[1..s.len() * 2 - 1].contains(&s);
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::repeated_substring_pattern("abab".to_owned())
    );
}
