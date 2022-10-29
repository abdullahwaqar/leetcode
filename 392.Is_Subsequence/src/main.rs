struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        if s.len() > t.len() {
            return false;
        }

        let char_buffer: Vec<char> = s.chars().collect();
        let mut idx_ptr: usize = 0;

        for val in t.chars() {
            if char_buffer[idx_ptr] == val {
                // * Increment pointer
                idx_ptr += 1;
                if idx_ptr == s.len() {
                    return true;
                }
            }
        }

        return false;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::is_subsequence("abc".to_owned(), "ahbgdc".to_owned())
    );
    println!(
        "{:?}",
        Solution::is_subsequence("ace".to_owned(), "abcde".to_owned())
    );
}
