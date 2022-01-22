use std::{collections::HashSet, ops::Add};

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        };


        // Small iq solution because i am not good at rust
        strs.sort_by(|a, b| a.len().cmp(&b.len()));
        println!("{:?}", strs);
        let mut result = String::new();

        for idx in 0..strs[0].len() {
            let character = strs[0].chars().nth(idx).unwrap();
            for str_buffer_idx in 0..strs.len() {
                if strs[str_buffer_idx].chars().nth(idx).unwrap() != character {
                    return result;
                }
            }
            result = format!("{}{}", result, character);
        }

        return result;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::longest_common_prefix(
            Vec::from(["flower", "flow", "flight"])
                .iter()
                .map(|elem| elem.to_string())
                .collect()
        )
    );
}
