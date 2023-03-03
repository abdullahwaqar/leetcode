use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.matches(' ').count() + 1 {
            return false;
        }

        let mut lookup_dict: HashMap<char, String> = HashMap::with_capacity(pattern.len());
        let mut word_set = HashSet::new();
        let s_buffer = s.split_whitespace().collect::<Vec<&str>>();
        let pattern_buffer = pattern.chars().collect::<Vec<_>>();

        for idx in 0..s_buffer.len() {
            println!("{:?}", pattern_buffer[idx]);

            if let Some(word) = lookup_dict.insert(pattern_buffer[idx], s_buffer[idx].to_owned()) {
                if word != s_buffer[idx] {
                    return false;
                }
            } else if !word_set.insert(s_buffer[idx]) {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned())
    );
    println!(
        "{:?}",
        Solution::word_pattern("abba".to_owned(), "dog cat cat fish".to_owned())
    );
}
