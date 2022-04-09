struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // Index of the character [a = 0, b = 1, .....]
        let idx = |c: u8| (c - 'a' as u8) as usize;

        let mut c1 = [0u8; 26];
        for c in s1.chars() {
            c1[idx(c as u8)] += 1;
        }

        // Counter for the sliding window on s2
        let mut c2 = [0u8; 26];
        // ! no Option<char>:
        let s2: &[u8] = s2.as_bytes();

        for i in 0..s2.len() {
            c2[idx(s2[i])] += 1;

            if i >= s1.len() - 1 {
                if c1 == c2 {
                    return true;
                }
                c2[idx(s2[i - s1.len() + 1])] -= 1;
            }
        }
        return false;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string())
    );
}
